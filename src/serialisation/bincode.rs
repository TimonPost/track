use crate::{preclude::SerdeDiff, serialisation::SerialisationStrategy};
use serde::{Deserialize, Serialize};
use std::result;

#[derive(Clone, Debug)]
/// Serialization strategy using bincode.
pub struct Bincode;

impl SerialisationStrategy for Bincode {
    fn serialize<I: Serialize>(&self, input: &I) -> Vec<u8> {
        bincode::serialize(&input).unwrap()
    }

    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> result::Result<T, ()> {
        bincode::deserialize::<T>(buffer).map_err(|_| ())
    }

    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]) {
        bincode::config()
            .deserialize_seed(serde_diff::Apply::deserializable(component), data)
            .unwrap();
    }
}

impl Default for Bincode {
    fn default() -> Self {
        Bincode
    }
}
