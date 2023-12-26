use std::error::Error;
use std::str::FromStr;

use sqlx::encode::IsNull;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};

use crate::pksuid::PrefixedKsuid;

impl Type<Postgres> for PrefixedKsuid {
	fn type_info() -> PgTypeInfo {
		PgTypeInfo::with_name("prefixedksuid")
	}
}

impl Encode<'_, Postgres> for PrefixedKsuid {
	fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
		<String as Encode<Postgres>>::encode(self.to_string(), buf)
	}
}

impl Decode<'_, Postgres> for PrefixedKsuid {
	fn decode(value: PgValueRef<'_>) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
		let s: &str = Decode::<Postgres>::decode(value)?;

		PrefixedKsuid::from_str(&s)
	}
}
