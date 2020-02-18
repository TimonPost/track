use crate::error::ErrorKind;
use crate::{serialisation::SerialisationStrategy, SerdeDiff};
use std::fmt::Debug;

/// Applies modified values to a type.
pub struct Apply;

impl Apply {
    /// Applies modified values to a type.
    ///
    /// * `type`: the type to which you want to apply the modified values.
    /// * `data`: the buffer with the modified type values.
    /// * `strategy`: the strategy used to deserialize the given `data` into the given `type`.
    pub fn apply_to<
        C: Clone + SerdeDiff + Debug + Send + Sync + SerdeDiff + 'static,
        S: SerialisationStrategy,
    >(
        component: &mut C,
        data: &[u8],
        strategy: S,
    ) -> Result<(), ErrorKind> {
        strategy.apply_to(component, data)
    }
}
