use core::ffi::CStr;
use std::str::FromStr;

use pgrx::pg_sys::StringInfoData;
use pgrx::prelude::*;
use pgrx::StringInfo;
use pksuid::error::BoxDynError;
use pksuid::Pksuid;

pgrx::pg_module_magic!();

#[pg_extern(immutable, parallel_safe, requires = ["shell_type"])]
fn pksuid_out<'a>(value: Pksuid) -> &'a CStr {
	let mut s = StringInfo::new();
	s.push_str(&value.to_string());
	s.into()
}

#[pg_extern(immutable, parallel_safe, requires = ["shell_type"])]
fn pksuid_in(input: &CStr) -> Result<Pksuid, BoxDynError> {
	Pksuid::from_str(input.to_str()?)
}

#[pg_extern(immutable, parallel_safe, requires = ["shell_type"])]
fn pksuid_send(input: Pksuid) -> Vec<u8> {
	input.into()
}

#[pg_extern(immutable, parallel_safe, requires = ["shell_type"])]
fn pksuid_receive(internal: pgrx::Internal) -> Result<Pksuid, BoxDynError> {
	let string_info = unsafe {
		let data = internal
			.get_mut::<StringInfoData>()
			.expect("internal doesnt point to StringInfoData");
		StringInfo::from_pg(data).expect("pointer is invalid")
	};

	Pksuid::from_str(&string_info.to_string())
}

#[pg_extern(immutable, parallel_safe, requires = ["shell_type"])]
fn pksuid_generate(prefix: &str) -> Pksuid {
	Pksuid::new(prefix.to_string())
}

extension_sql!("CREATE TYPE pksuid; -- shell type", name = "shell_type", bootstrap);

extension_sql!(
	r#"
create type pksuid (
	input = pksuid_in,
	output = pksuid_out,
	receive = pksuid_receive,
	send = pksuid_send,
	like = text
);"#,
	name = "concrete_type",
	creates = [Type(Pksuid)],
	requires = ["shell_type", pksuid_in, pksuid_out, pksuid_send, pksuid_receive],
);

// casts
#[pg_extern(immutable, parallel_safe)]
fn text_to_pksuid(input: &str) -> Result<Pksuid, BoxDynError> {
	Pksuid::from_str(input)
}

#[pg_extern(immutable, parallel_safe)]
fn pksuid_to_text(input: Pksuid) -> String {
	input.to_string()
}

extension_sql!(
	r#"
create cast (text AS pksuid) with function text_to_pksuid as implicit;
create cast (pksuid AS text) with function pksuid_to_text as implicit;
"#,
	name = "casts",
	requires = [text_to_pksuid, pksuid_to_text],
);

#[cfg(not(feature = "no-schema-generation"))]
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use std::error::Error;

	use pgrx::prelude::*;
	use pksuid::Pksuid;

	#[pg_test]
	fn test_pksuid_select() -> Result<(), Box<dyn Error>> {
		let value = Spi::get_one::<Pksuid>("select 'client_2a3Hg5Z5sAk7Armrs7qaKMxdE17'::Pksuid;")?;

		assert_eq!(
			value,
			Some(Pksuid {
				prefix: "client".to_string(),
				ksuid: "2a3Hg5Z5sAk7Armrs7qaKMxdE17".parse().unwrap(),
			})
		);

		Ok(())
	}

	#[pg_test]
	fn test_pksuid_generate() -> Result<(), Box<dyn Error>> {
		let value = Spi::get_one::<Pksuid>("select pksuid_generate('client');")?;

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
