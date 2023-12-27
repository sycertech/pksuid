use std::str::FromStr;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::Pksuid;

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

		Self::from_str(&s).map_err(serde::de::Error::custom)
	}
}
