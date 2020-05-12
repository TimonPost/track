//! Module with different serializers that can be used with this crate.
//!
//! By default, a number of serializers are supplied that can be used by turning on the feature flag.
//! You might want to create your own serializing strategy by implementing: [SerializationStrategy](./trait.SerializationStrategy.html).
//!
//! | Feature | Description |
//! | :----- | :----- |
//! | `bincode-serialization` | serialization using [bincode](https://crates.io/crates/bincode) (enabled by default) .|
//! | `rmp-serialization` | serialization using [rmp-serde](https://crates.io/crates/rmp-serde) .|

use serde::{Deserialize, Serialize};

use crate::error::ErrorKind;
use crate::preclude::SerdeDiff;

/// Implementation of [SerializationStrategy](./trait.SerializationStrategy.html) for serializing with [bincode](https://crates.io/crates/bincode).
/// It is enabled with the `bincode-serialization` feature flag.
///
/// - `bincode-serialization` is enabled by default.
#[cfg(feature = "bincode-serialization")]
pub mod bincode;

/// Implementation of [SerializationStrategy](./trait.SerializationStrategy.html) for serializing with [rmp-serde](https://crates.io/crates/rmp-serde).
/// It is enabled with the `rmp-serialization` feature flag.
#[cfg(feature = "rmp-serialization")]
pub mod rmp;

/// An adapter interface with extension methods for serializing purposes used in this crate.
pub trait SerializationStrategy: Clone + Default + Send + Sync {
    /// Serializes the given type to a byte buffer.
    fn serialize<I: Serialize>(&self, input: &I) -> Result<Vec<u8>, ErrorKind>;

    /// Deserializes the given byte buffer to the desired type.
    fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> Result<T, ErrorKind>;

    /// Applies the given byte buffer to the given type.
    /// The buffer contains the data of the modified fields sent with the [ModificationEvent](../../track/struct.ModificationEvent.html).
    fn apply_to<C: SerdeDiff>(&self, component: &mut C, data: &[u8]) -> Result<(), ErrorKind>;
}

/// A wrapper type over an implementation of [SerializationStrategy](./trait.SerializationStrategy.html).
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct ModificationSerializer<S: SerializationStrategy> {
    strategy: S,
}

impl<S: SerializationStrategy> ModificationSerializer<S> {
    pub fn new(strategy: S) -> ModificationSerializer<S> {
        ModificationSerializer { strategy }
    }

    pub fn serialize<I: Serialize>(&self, input: &I) -> Result<Vec<u8>, ErrorKind> {
        self.strategy.serialize(input)
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(&self, buffer: &'a [u8]) -> Result<T, ErrorKind> {
        self.strategy.deserialize(buffer)
    }
}

impl<S: SerializationStrategy> Default for ModificationSerializer<S> {
    fn default() -> Self {
        ModificationSerializer::new(Default::default())
    }
}
