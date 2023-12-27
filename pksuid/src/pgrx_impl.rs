use std::str::FromStr;

use ::pgrx::pg_sys::{Datum, Oid};
use ::pgrx::pgrx_sql_entity_graph::metadata::{ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable};
use ::pgrx::{rust_regtypein, FromDatum, IntoDatum};

use crate::Pksuid;

unsafe impl SqlTranslatable for Pksuid {
	fn argument_sql() -> Result<SqlMapping, ArgumentError> {
		// this is what the SQL type is called when used in a function argument position
		Ok(SqlMapping::As("pksuid".into()))
	}

	fn return_sql() -> Result<Returns, ReturnsError> {
		// this is what the SQL type is called when used in a function return type
		// position
		Ok(Returns::One(SqlMapping::As("pksuid".into())))
	}
}

impl FromDatum for Pksuid {
	unsafe fn from_polymorphic_datum(datum: Datum, is_null: bool, typoid: Oid) -> Option<Self>
	where Self: Sized {
		if is_null {
			None
		} else {
			let string = String::from_polymorphic_datum(datum, is_null, typoid).unwrap();

			Self::from_str(&string).ok()
		}
	}
}

impl IntoDatum for Pksuid {
	fn into_datum(self) -> Option<Datum> {
		self.to_string().into_datum()
	}

	fn type_oid() -> Oid {
		rust_regtypein::<Self>()
	}
}

// #[cfg(any(
// 	feature = "pg11",
// 	feature = "pg12",
// 	feature = "pg13",
// 	feature = "pg14",
// 	feature = "pg15",
// 	feature = "pg16"
// ))]
// impl pgrx::InOutFuncs for Pksuid {
// 	fn input(input: &core::ffi::CStr) -> Self
// 	where Self: Sized {
// 		let mut parts = input.to_str().unwrap().split('_');
// 		let (prefix, ksuid) = (
// 			parts.next().expect("prefix missing"),
// 			parts.next().expect("ksuid missing"),
// 		);

// 		Self {
// 			prefix: prefix.to_string(),
// 			ksuid: Ksuid::from_str(ksuid).unwrap(),
// 		}
// 	}

// 	fn output(&self, buffer: &mut pgrx::StringInfo) {
// 		buffer.push_str(&format!("{}_{}", self.prefix, self.ksuid));
// 	}
// }
