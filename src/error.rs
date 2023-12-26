use std::error::Error as StdError;

pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;
