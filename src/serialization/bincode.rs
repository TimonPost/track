use crate::error::ErrorKind;
use crate::{preclude::SerdeDiff, serialization::SerializationStrategy};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug)]
/// Serialization strategy using bincode.
pub struct Bincode;

impl SerializationStrategy for Bincode {
    fn serialize<I: Serialize>(&self, input: &I) -> Result<Vec<u8>, ErrorKind> {
        Ok(bincode::serialize(&input)
            .map_err(|e| ErrorKind::SerializationError(e.description().to_string()))?)
    }

    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> Result<T, ErrorKind> {
        Ok(bincode::deserialize::<T>(buffer)
            .map_err(|e| ErrorKind::SerializationError(e.description().to_string()))?)
    }

    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]) -> Result<(), ErrorKind> {
        bincode::config()
            .deserialize_seed(serde_diff::Apply::deserializable(component), data)
            .map_err(|e| ErrorKind::SerializationError(e.description().to_string()))?;

        Ok(())
    }
}

impl Default for Bincode {
    fn default() -> Self {
        Bincode
    }
}
