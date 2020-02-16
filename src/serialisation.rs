//! Module with different serializers that can be used with this crate.
//!
//! By default, a number of serializers are supplied that can be used by turning on the feature flag.
//! You might want to create your own serializing strategy by implementing: [SerialisationStrategy](LINK).
//!
//! | Feature | Description |
//! | :-- | :-- |
//! | `bincode-serialization` | serialisation using [bincode](LINK) (enabled by default) .|
//! | `rmp-serialization` | serialisation using [rmp-serde](LINK) .|

use std::result;

use crate::preclude::SerdeDiff;
use serde::{Deserialize, Serialize};

/// Implementation of [SerialisationStrategy](LINK) for serializing with [bincode](LINK).
/// It is enabled with the `bincode-serialization` feature flag.
///
/// - `bincode-serialization` is enabled by default.
#[cfg(feature = "bincode-serialization")]
pub mod bincode;

/// Implementation of [SerialisationStrategy](LINK) for serializing with [rmp-serde]().
/// It is enabled with the `rmp-serialization` feature flag.
#[cfg(feature = "rmp-serialization")]
pub mod rmp;

/// An adapter interface with extension methods for serializing purposes used in this crate.
pub trait SerialisationStrategy: Clone + Default {
    /// Serializes the given type to a byte buffer.
    fn serialize<I: Serialize>(&self, input: &I) -> Vec<u8>;

    /// Deserializes the given byte buffer to the desired type.
    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> result::Result<T, ()>;

    /// Applies the given byte buffer to the given type.
    /// The buffer contains the data of the modified fields sent with the [ModificationEvent](LINK).
    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]);
}

/// A wrapper type over an implementation of [SerialisationStrategy](LINK).
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct ModificationSerializer<S: SerialisationStrategy> {
    strategy: S,
}

impl<S: SerialisationStrategy> ModificationSerializer<S> {
    pub fn new(strategy: S) -> ModificationSerializer<S> {
        ModificationSerializer { strategy }
    }

    pub fn serialize<I: Serialize>(&self, input: &I) -> Vec<u8> {
        self.strategy.serialize(input)
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> result::Result<T, ()> {
        self.strategy.deserialize(buffer)
    }
}

impl<S: SerialisationStrategy> Default for ModificationSerializer<S> {
    fn default() -> Self {
        ModificationSerializer::new(Default::default())
    }
}
