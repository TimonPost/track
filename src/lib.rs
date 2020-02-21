//! This library offers a boilerplate free approach to track struct data modifications.
//! For optimization, only the adjusted fields are tracked. Changes will be serialized and sent on an channel.
//!
//! ## Examples
//!
//! First, add `track` attribute to mark your struct as trackable.
//! ```rust
//! // imports all necessarily types for the `track` attribute.
//! use track::preclude::*;
//!
//! #[track]
//! #[derive(Debug)]
//! pub struct Position {
//!     pub x: u32,
//!     pub y: u32,
//! }
//! ```
//!
//! You can specify a serialization method for the track macro.
//! Give the name of the type that implements [SerializationStrategy](https://docs.rs/track/serialization/trait.SerializationStrategy.html), and make sure it is in scope for the macro.
//! Such as:
//!
//! ```rust
//! use track::serialization::bincode::Bincode;
//!
//! #[track(serialization = "Bincode")]
//! struct Postition ...
//! ```
//!
//! Now let us make some modifications and apply those to other instances.
//! ```rust
//! use track::{preclude::*, serialization::bincode::Bincode, Apply, ModificationChannel};
//!
//! #[derive(Copy, Clone, Debug, PartialEq)]
//! pub struct Identity {
//!     pub value: u8,
//! }
//!
//! impl Identifier for Identity {}
//!
//! fn main() {
//!     let channel = ModificationChannel::<Identity>::new();
//!
//!     let updated_storage = vec![
//!         (Identity { value: 1 }, Position { x: 0, y: 0 }),
//!         (Identity { value: 2 }, Position { x: 0, y: 0 }),
//!     ];
//!     let mut outdated_storage = updated_storage.clone();
//!
//!     // == Make changes to storage ==
//!     make_changes(&channel, updated_storage);
//!
//!     // == Apply changes to outdated storage ==
//!     apply_changes(&channel, &mut outdated_storage);
//! }
//!
//! fn make_changes(channel: &ModificationChannel<Identity>, entities: Vec<(Identity, Position)>) {
//!     for (id, mut position) in entities {
//!         let mut position = position.track(channel.sender(), id); /* returns `Tracker` which tracks changes */
//!
//!         // `Tracker` implements `DerefMut`
//!         position.x += 1;
//!         position.y += 1;
//!     } // <- on the `Drop` of `wrapper` changes are serialized and sent on the channel.
//! }
//!
//! fn apply_changes(
//!     channel: &ModificationChannel<Identity>,
//!     entities: &mut Vec<(Identity, Position)>,
//! ) {
//!     for event in channel.receiver().try_iter() {
//!         let entity = entities
//!             .iter_mut()
//!             .find(|e| e.0 == event.identifier)
//!             .unwrap();
//!
//!         Apply::apply_to(&mut entity.1, &event.modified_fields, Bincode);
//!
//!         println!("entity updated {:?}", entity);
//!     }
//! }
//! ```
//!
//! _For a more in-depth example checkout the [examples](https://github.com/entity-sync-rs/track/tree/master/examples) on github._

use crossbeam_channel::Sender;
use serde_diff::SerdeDiff;

pub use self::{
    apply::Apply, channel::ModificationChannel, event::ModificationEvent, tracker::Tracker,
};
pub use track_macro::track;

use std::fmt::Debug;

mod apply;
mod channel;
pub mod error;
mod event;
mod tracker;

pub mod serialization;

/// A trait with functions for tracking struct value modifications.
///
/// Do not implement this trait manually but use the `track` attribute for less boiler plate code.
pub trait Trackable<C, S>
where
    C: TrackableMarker,
    S: serialization::SerializationStrategy,
{
    fn track<'notifier, I: Identifier>(
        &mut self,
        sender: &'notifier Sender<ModificationEvent<I>>,
        identifier: I,
    ) -> Tracker<'_, 'notifier, C, S, I>;
}

/// A marker trait with a number of requirements that are mandatory for trackable types.
pub trait TrackableMarker: Clone + SerdeDiff + Debug + Send + Sync + 'static {}

/// A marker trait witch should be implemented for types used as identity in the [Tracker](./struct.Tracker.html).
pub trait Identifier: Copy + Clone + Send + Sync {}

pub mod re_exports {
    /// A re-export of the [serde](https://crates.io/crates/serde) create.
    pub mod serde {
        pub use serde::*;
    }

    /// A re-export of the [serde-diff](https://crates.io/crates/serde-diff) create.
    pub mod serde_diff {
        pub use serde_diff::*;
    }

    /// A re-export of the [crossbeam-channel](https://crates.io/crates/crossbeam-channel) create.
    pub mod crossbeam_channel {
        pub use crossbeam_channel::*;
    }
}

/// A re-export with types needed for the [track](./struct.Tracker.html) attribute.
pub mod preclude {
    pub use crate::{Identifier, ModificationEvent, Trackable, TrackableMarker, Tracker};

    pub use self::serde_diff::SerdeDiff;
    pub use crossbeam_channel::Sender;
    pub use serde::{Deserialize, Serialize};

    pub use track_macro::track;

    pub use crate::serialization::{bincode::Bincode, SerializationStrategy};

    // [serde-diff](https://crates.io/crates/serde-diff)s macro's require `serde_diff` to be imported when we use `track` attribute macro.
    pub use crate::re_exports::serde_diff;
}
