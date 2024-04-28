use crate::prelude::*;
use crate::utils;

use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use toml;

/// Serializing/deserializing data to/from TOML-configs
pub trait Toml {
    /// Reading TOML-config and deserializing it into `Self` object
    fn parse<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let contents = utils::read_to_string(&path)?;
        let data = toml::from_str::<Self>(&contents)
            .map_err(|err| Error::new(err).set_kind(ErrorKind::DeserializationError))?;
        Ok(data)
    }

    /// Writing a serialized object `Self` to the TOML config file
    fn write<P>(&self, path: P) -> Result<()>
    where
        Self: Serialize,
        P: AsRef<Path>,
    {
        let data = toml::to_string(&self)
            .map_err(|err| Error::new(err).set_kind(ErrorKind::SerializationError))?;
        utils::write(&path, data)
    }
}
