use crossbeam_channel::Sender;
use serde_diff::SerdeDiff;
use uuid::Uuid;

pub use self::{
    apply::Apply, channel::ModificationChannel, event::ModificationEvent, tracker::Tracker,
};
pub use track_macro::track;

use std::fmt::Debug;

mod apply;
mod channel;
mod event;
mod tracker;

pub mod serialisation;

/// A trait with functions for tracking struct value modifications.
///
/// Do not implement this trait manually but use the `track` attribute for less boiler plate code.
pub trait Trackable<C, S>
where
    C: TrackableMarker,
    S: serialisation::SerialisationStrategy,
{
    fn track<'notifier>(
        &mut self,
        sender: &'notifier Sender<ModificationEvent>,
    ) -> Tracker<'_, 'notifier, C, S>;

    fn track_by<'notifier>(
        &mut self,
        sender: &'notifier Sender<ModificationEvent>,
        entity: Uuid,
    ) -> Tracker<'_, 'notifier, C, S>;
}

/// A marker traits with a number of requirements that are mandatory for trackable types.
pub trait TrackableMarker: Clone + SerdeDiff + Debug + Send + Sync + 'static {}

pub mod re_exports {
    /// A re-export of the [serde](LINK) create.
    pub mod serde {
        pub use serde::*;
    }

    /// A re-export of the [serde-diff](LINK) create.
    pub mod serde_diff {
        pub use serde_diff::*;
    }

    /// A re-export of the [crossbeam-channel](LINK) create.
    pub mod crossbeam_channel {
        pub use crossbeam_channel::*;
    }

    /// A re-export of the [uuid](LINK) create.
    pub mod uuid {
        pub use uuid::*;
    }
}

/// A re-export with types needed for the [track](LINK) attribute.
pub mod preclude {
    pub use crate::{
        ModificationEvent, Trackable, TrackableMarker, Tracker,
    };

    pub use crossbeam_channel::Sender;
    pub use serde::{Deserialize, Serialize};
    pub use self::serde_diff::SerdeDiff;
    pub use uuid::Uuid;

    pub use track_macro::track;

    pub use crate::serialisation::{bincode::Bincode, SerialisationStrategy};

    // `serde-diff`s macro's require `serde_diff` to be imported when we use `track` attribute macro.
    pub use crate::re_exports::serde_diff;
}
