use serde::de::{Deserialize, Deserializer};

pub fn deserialize_some<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::<T>::deserialize(deserializer)? {
        Some(v) => Ok(Some(Some(v))),
        None => Ok(Some(None)),
    }
}
