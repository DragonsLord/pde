use std::path::PathBuf;

use serde::Deserialize;

use crate::utils::path_extensions::PathExtensions;

pub struct ParseUtils {}
impl ParseUtils {
    pub fn parse_path<'de, D>(d: D) -> Result<PathBuf, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de;
        let path = PathBuf::deserialize(d)?;
        path.pde_resolve().map_err(de::Error::custom)
    }

    pub fn parse_optional_path<'de, D>(d: D) -> Result<Option<PathBuf>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de;
        let value = Option::<PathBuf>::deserialize(d)?;
        Ok(match value {
            Some(path) => Some(path.pde_resolve().map_err(de::Error::custom)?),
            None => None,
        })
    }
}
