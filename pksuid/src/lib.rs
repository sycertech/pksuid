use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use error::BoxDynError;
use svix_ksuid::{Ksuid, KsuidLike};

pub mod error;
#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod serde;

pub mod pgrx_impl;

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
	derive(::pgrx::PostgresOrd, ::pgrx::PostgresHash, ::pgrx::PostgresEq,)
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

impl Pksuid {
	pub fn new(prefix: String) -> Self {
		Self {
			prefix,
			ksuid: Ksuid::new(None, None),
		}
	}
}

impl FromStr for Pksuid {
	type Err = BoxDynError;

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

impl Into<Vec<u8>> for Pksuid {
	fn into(self) -> Vec<u8> {
		self.to_string().into_bytes()
	}
}
