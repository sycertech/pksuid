use std::{fmt::Display, str::FromStr};
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Clone, Debug)]
pub struct PrefixedKsuid {
    pub prefix: String,
    pub ksuid: Ksuid,
}

impl Display for PrefixedKsuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.prefix, self.ksuid)
    }
}

impl Ord for PrefixedKsuid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ksuid.cmp(&other.ksuid)
    }
}

impl PartialOrd for PrefixedKsuid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ksuid.partial_cmp(&other.ksuid)
    }
}

impl PartialEq for PrefixedKsuid {
    fn eq(&self, other: &Self) -> bool {
        self.ksuid == other.ksuid
    }
}

impl Eq for PrefixedKsuid {}

// impl InOutFuncs for PrefixedKsuid {
// 	fn input(input: &core::ffi::CStr) -> Self
// 		where
// 			Self: Sized {
// 		let mut parts = input.to_str().unwrap().split('_');
// 		let (prefix, ksuid) = (parts.next().unwrap(), parts.next().unwrap());

// 		Self {
// 			prefix: prefix.to_string(),
// 			ksuid: Ksuid::from_str(ksuid).unwrap(),
// 		}
// 	}

// 	fn output(&self, buffer: &mut pgrx::StringInfo) {
// 		buffer.push_str(&format!("{}_{}", self.prefix, self.ksuid));
// 	}
// }

impl serde::ser::Serialize for PrefixedKsuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}_{}", self.prefix, self.ksuid))
    }
}

impl<'de> serde::de::Deserialize<'de> for PrefixedKsuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('_');
        let prefix = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing prefix"))?;
        let ksuid = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing ksuid"))?;

        Ok(Self {
            prefix: prefix.to_string(),
            ksuid: Ksuid::from_str(ksuid).map_err(serde::de::Error::custom)?,
        })
    }
}

impl PrefixedKsuid {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            ksuid: Ksuid::new(None, None),
        }
    }
}

impl FromStr for PrefixedKsuid {
    // TODO: better error
    type Err = Box<dyn std::error::Error>;

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
