use pgrx::prelude::*;
use pksuid::Pksuid;

pgrx::pg_module_magic!();

#[pg_extern(immutable, parallel_safe)]
fn pksuid_generate(prefix: &str) -> Pksuid {
	Pksuid::new(prefix.to_string())
}

#[cfg(not(feature = "no-schema-generation"))]
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use std::error::Error;

	use pgrx::prelude::*;
	use pksuid::Pksuid;

	#[pg_test]
	fn test_pksuid_select() -> Result<(), Box<dyn Error>> {
		let value = Spi::get_one::<Pksuid>("select 'client_2a3Hg5Z5sAk7Armrs7qaKMxdE17'::prefixedksuid;")?;

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
		let value = Spi::get_one::<Pksuid>("select prefixedksuid_generate('client');")?;

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
