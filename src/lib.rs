use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use pgrx::rust_regtypein;
use pgrx::{pg_sys::Datum, prelude::*};
use pgrx::StringInfo;
use pksuid::PrefixedKsuid;
use std::str::FromStr;
use std::{error::Error, ffi::CStr};
use svix_ksuid::Ksuid;

pgrx::pg_module_magic!();

pub mod pksuid;

unsafe impl SqlTranslatable for PrefixedKsuid {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        // this is what the SQL type is called when used in a function argument position
        Ok(SqlMapping::As("prefixed_ksuid".into()))
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        // this is what the SQL type is called when used in a function return type position
        Ok(Returns::One(SqlMapping::As("prefixed_ksuid".into())))
    }
}

impl FromDatum for PrefixedKsuid {
    unsafe fn from_polymorphic_datum(
        datum: Datum,
        is_null: bool,
        typoid: pgrx::pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
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
fn prefixed_ksuid_out<'a>(value: PrefixedKsuid) -> &'a CStr {
    let mut s = StringInfo::new();
    s.push_str(&value.to_string());
    s.into()
}

#[pg_extern(immutable, parallel_safe, requires = [ "shell_type" ])]
fn prefixed_ksuid_in(input: &CStr) -> Result<PrefixedKsuid, Box<dyn Error>> {
    let mut parts = input.to_str()?.split('_');
    let (prefix, ksuid) = (parts.next().unwrap(), parts.next().unwrap());

    Ok(PrefixedKsuid {
        prefix: prefix.to_string(),
        ksuid: Ksuid::from_str(ksuid)?,
    })
}

#[pg_extern(immutable, parallel_safe, requires = [ "concrete_type" ])]
fn create_prefixed_ksuid(prefix: &str) -> PrefixedKsuid {
    PrefixedKsuid::new(prefix.to_string())
}

// creates the `hexint` shell type, which is essentially a type placeholder so that the
// input and output functions can be created
extension_sql!(
    r#"CREATE TYPE prefixed_ksuid; -- shell type"#,
    name = "shell_type",
    bootstrap
);

extension_sql!(
    r#"
CREATE TYPE prefixed_ksuid (
    INPUT = prefixed_ksuid_in,
    OUTPUT = prefixed_ksuid_out,
    LIKE = text
);
"#,
    name = "concrete_type",
    creates = [Type(PrefixedKsuid)],
    requires = ["shell_type", prefixed_ksuid_in, prefixed_ksuid_out],
);

#[cfg(not(feature = "no-schema-generation"))]
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use std::error::Error;
    use pgrx::prelude::*;
    use crate::pksuid::PrefixedKsuid;

    #[pg_test]
    fn test_prefixed_ksuid_input() -> Result<(), Box<dyn Error>> {
        let value = Spi::get_one::<PrefixedKsuid>(
            "SELECT 'client_2a3Hg5Z5sAk7Armrs7qaKMxdE17'::prefixed_ksuid;",
        )?;

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
    fn test_prefixed_ksuid_output() -> Result<(), Box<dyn Error>> {
        let value = Spi::get_one::<String>(
            "SELECT 'client_2a3Hg5Z5sAk7Armrs7qaKMxdE17'::prefixed_ksuid;",
        )?;

        assert_eq!(value, Some("client_2a3Hg5Z5sAk7Armrs7qaKMxdE17".to_string()));

        Ok(())
    }

    #[pg_test]
    fn test_prefixed_ksuid_create() -> Result<(), Box<dyn Error>> {
        let value = Spi::get_one::<PrefixedKsuid>(
            "SELECT create_prefixed_ksuid('client');",
        )?;

        assert_eq!(
            "client",
            value
                .map(|v| v.prefix)
                .ok_or("missing value")?
        );

        Ok(())
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
