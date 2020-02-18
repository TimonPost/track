use crate::error::ErrorKind;
use crate::{preclude::SerdeDiff, serialisation::SerialisationStrategy};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug)]
/// Serialization strategy using rmp-serde.
pub struct Rmp;

impl SerialisationStrategy for Rmp {
    fn serialize<I: Serialize>(&self, input: &I) -> Result<Vec<u8>, ErrorKind> {
        Ok(rmp_serde::to_vec(&input)
            .map_err(|e| ErrorKind::SerialisationError(e.description().to_string()))?)
    }

    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> Result<T, ErrorKind> {
        let mut de = rmp_serde::Deserializer::from_read_ref(buffer);
        Ok(Deserialize::deserialize(&mut de)
            .map_err(|e| ErrorKind::SerialisationError(e.description().to_string()))?)
    }

    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]) -> Result<(), ErrorKind> {
        let mut deserializer = rmp_serde::Deserializer::new(data);
        serde_diff::Apply::apply(&mut deserializer, component)
            .map_err(|e| ErrorKind::SerialisationError(e.description().to_string()))?;

        Ok(())
    }
}

impl Default for Rmp {
    fn default() -> Self {
        Rmp
    }
}
