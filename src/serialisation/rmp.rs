use crate::{preclude::SerdeDiff, serialisation::SerialisationStrategy};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
/// Serialization strategy using rmp-serde.
pub struct Rmp;

impl SerialisationStrategy for Rmp {
    fn serialize<I: Serialize>(&self, input: &I) -> Vec<u8> {
        rmp_serde::to_vec(&input).unwrap()
    }

    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> Result<T, ()> {
        let mut de = rmp_serde::Deserializer::from_read_ref(buffer);
        Deserialize::deserialize(&mut de).map_err(|_| ())
    }

    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]) {
        let mut deserializer = rmp_serde::Deserializer::new(data);
        serde_diff::Apply::apply(&mut deserializer, component).unwrap();
    }
}

impl Default for Rmp {
    fn default() -> Self {
        Rmp
    }
}
