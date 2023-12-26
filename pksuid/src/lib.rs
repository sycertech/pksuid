use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use svix_ksuid::{Ksuid, KsuidLike};

pub mod error;
#[cfg(feature = "sqlx")]
pub mod sqlx;

// todo: this
// #[cfg(all(feature = "sqlx", feature = "__pg"))]
// compile_error!("The features `sqlx` and `pgXX` are mutually exclusive.");

#[cfg_attr(
	any(
		feature = "pg11",
		feature = "pg12",
		feature = "pg13",
		feature = "pg14",
		feature = "pg15",
		feature = "pg16"
	),
	derive(pgrx::PostgresOrd, pgrx::PostgresHash, pgrx::PostgresEq, pgrx::PostgresType),
	inoutfuncs
)]
#[derive(Clone, Debug)]
pub struct Pksuid {
	pub prefix: String,
	pub ksuid: Ksuid,
}

impl std::hash::Hash for Pksuid {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.prefix.hash(state);
		self.ksuid.bytes().hash(state);
	}
}

impl Display for Pksuid {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}_{}", self.prefix, self.ksuid)
	}
}

impl Ord for Pksuid {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		self.ksuid.cmp(&other.ksuid)
	}
}

impl PartialOrd for Pksuid {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Pksuid {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.ksuid == other.ksuid
	}
}

impl Eq for Pksuid {}

#[cfg(any(
	feature = "pg11",
	feature = "pg12",
	feature = "pg13",
	feature = "pg14",
	feature = "pg15",
	feature = "pg16"
))]
impl pgrx::InOutFuncs for Pksuid {
	fn input(input: &core::ffi::CStr) -> Self
	where Self: Sized {
		let mut parts = input.to_str().unwrap().split('_');
		let (prefix, ksuid) = (
			parts.next().expect("prefix missing"),
			parts.next().expect("ksuid missing"),
		);

		Self {
			prefix: prefix.to_string(),
			ksuid: Ksuid::from_str(ksuid).unwrap(),
		}
	}

	fn output(&self, buffer: &mut pgrx::StringInfo) {
		buffer.push_str(&format!("{}_{}", self.prefix, self.ksuid));
	}
}

impl Serialize for Pksuid {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		serializer.serialize_str(&format!("{}_{}", self.prefix, self.ksuid))
	}
}

impl<'de> Deserialize<'de> for Pksuid {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let s = String::deserialize(deserializer)?;
		let mut parts = s.split('_');

		macro_rules! parse_part {
			($part:ident, $s:expr, $err:expr) => {
				let $part = $s.next().ok_or_else(|| serde::de::Error::custom($err))?;
			};
		}
		parse_part!(prefix, parts, "missing prefix");
		parse_part!(ksuid, parts, "missing ksuid");

		Ok(Self {
			prefix: prefix.to_string(),
			ksuid: Ksuid::from_str(ksuid).map_err(serde::de::Error::custom)?,
		})
	}
}

impl Pksuid {
	pub fn new(prefix: String) -> Self {
		Self {
			prefix,
			ksuid: Ksuid::new(None, None),
		}
	}
}

impl FromStr for Pksuid {
	type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split('_');
		let prefix = parts.next().ok_or("missing prefix")?;
		let ksuid = parts.next().ok_or("missing ksuid")?;

		Ok(Self {
			prefix: prefix.to_string(),
			ksuid: Ksuid::from_str(ksuid)?,
		})
	}
}
