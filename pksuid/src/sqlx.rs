use std::str::FromStr;

use sqlx::encode::IsNull;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};

use super::Pksuid;
use crate::error::BoxDynError;

impl Type<Postgres> for Pksuid {
	fn type_info() -> PgTypeInfo {
		PgTypeInfo::with_name("pksuid")
	}

	fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
		<String as Type<Postgres>>::compatible(ty)
	}
}

impl Encode<'_, Postgres> for Pksuid {
	fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
		<String as Encode<Postgres>>::encode(self.to_string(), buf)
	}
}

impl Decode<'_, Postgres> for Pksuid {
	fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
		let s: &str = Decode::<Postgres>::decode(value)?;

		Pksuid::from_str(&s)
	}
}
