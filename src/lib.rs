#![allow(clippy::extra_unused_lifetimes)]
use std::ffi::CStr;
use std::str::FromStr;

use pgrx::pg_sys::Datum;
use pgrx::pgrx_sql_entity_graph::metadata::{ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable};
use pgrx::prelude::*;
use pgrx::{rust_regtypein, StringInfo};
use pksuid::PrefixedKsuid;
use svix_ksuid::Ksuid;

use crate::error::BoxDynError;

pgrx::pg_module_magic!();

pub mod error;
pub mod pksuid;
pub mod sqlx;

unsafe impl SqlTranslatable for PrefixedKsuid {
	fn argument_sql() -> Result<SqlMapping, ArgumentError> {
		// this is what the SQL type is called when used in a function argument position
		Ok(SqlMapping::As("prefixedksuid".into()))
	}

	fn return_sql() -> Result<Returns, ReturnsError> {
		// this is what the SQL type is called when used in a function return type
		// position
		Ok(Returns::One(SqlMapping::As("prefixedksuid".into())))
	}
}

impl FromDatum for PrefixedKsuid {
	unsafe fn from_polymorphic_datum(datum: Datum, is_null: bool, typoid: pgrx::pg_sys::Oid) -> Option<Self>
	where Self: Sized {
		if is_null {
			None
		} else {
			let string = String::from_polymorphic_datum(datum, is_null, typoid).unwrap();
			let mut parts = string.split('_');
			let (prefix, ksuid) = (parts.next().unwrap(), parts.next().unwrap());

			Some(Self {
				prefix: prefix.to_string(),
				ksuid: Ksuid::from_str(ksuid).unwrap(),
			})
		}
	}
}

impl IntoDatum for PrefixedKsuid {
	fn into_datum(self) -> Option<Datum> {
		self.to_string().into_datum()
	}

	fn type_oid() -> pgrx::pg_sys::Oid {
		rust_regtypein::<Self>()
	}
}

#[pg_extern(immutable, parallel_safe, requires = [ "shell_type" ])]
fn prefixedksuid_out<'a>(value: PrefixedKsuid) -> &'a CStr {
	let mut s = StringInfo::new();
	s.push_str(&value.to_string());

	s.into()
}

#[pg_extern(immutable, parallel_safe, requires = [ "shell_type" ])]
fn prefixedksuid_in(input: &CStr) -> Result<PrefixedKsuid, BoxDynError> {
	let mut parts = input.to_str()?.split('_');
	let (prefix, ksuid) = (parts.next().unwrap(), parts.next().unwrap());

	Ok(PrefixedKsuid {
		prefix: prefix.to_string(),
		ksuid: Ksuid::from_str(ksuid)?,
	})
}

#[pg_extern(immutable, parallel_safe, requires = [ "concrete_type" ])]
fn prefixedksuid_generate(prefix: &str) -> PrefixedKsuid {
	PrefixedKsuid::new(prefix.to_string())
}

// casts
#[pg_extern(immutable, parallel_safe)]
fn text_to_prefixedksuid(input: &str) -> Result<PrefixedKsuid, BoxDynError> {
	PrefixedKsuid::from_str(input)
}

#[pg_extern(immutable, parallel_safe)]
fn prefixedksuid_to_text(input: PrefixedKsuid) -> String {
	input.to_string()
}

extension_sql!(
	r#"create type prefixedksuid; -- shell type"#,
	name = "shell_type",
	bootstrap
);

extension_sql!(
	r#"
create type prefixedksuid (
    input = prefixedksuid_in,
    output = prefixedksuid_out,
    like = text
);
"#,
	name = "concrete_type",
	creates = [Type(PrefixedKsuid)],
	requires = ["shell_type", prefixedksuid_in, prefixedksuid_out],
);

extension_sql!(
	r#"
create cast (text AS prefixedksuid) with function text_to_prefixedksuid as implicit;
create cast (prefixedksuid AS text) with function prefixedksuid_to_text as implicit;
"#,
	name = "casts",
	requires = [text_to_prefixedksuid, prefixedksuid_to_text],
);

#[cfg(not(feature = "no-schema-generation"))]
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use std::error::Error;

	use pgrx::prelude::*;

	use crate::pksuid::PrefixedKsuid;

	#[pg_test]
	fn test_ksuid_select() -> Result<(), Box<dyn Error>> {
		let value = Spi::get_one::<PrefixedKsuid>("select 'client_2a3Hg5Z5sAk7Armrs7qaKMxdE17'::prefixedksuid;")?;

		assert_eq!(
			value,
			Some(PrefixedKsuid {
				prefix: "client".to_string(),
				ksuid: "2a3Hg5Z5sAk7Armrs7qaKMxdE17".parse().unwrap(),
			})
		);

		Ok(())
	}

	#[pg_test]
	fn test_prefixedksuid_generate() -> Result<(), Box<dyn Error>> {
		let value = Spi::get_one::<PrefixedKsuid>("select prefixedksuid_generate('client');")?;

		assert_eq!("client", value.map(|v| v.prefix).ok_or("missing value")?);

		Ok(())
	}
}

#[cfg(test)]
pub mod pg_test {
	pub fn setup(_options: Vec<&str>) {}

	pub fn postgresql_conf_options() -> Vec<&'static str> {
		vec![]
	}
}
