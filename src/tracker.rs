use std::ops::{Deref, DerefMut};

use crossbeam_channel::Sender;
use serde_diff::{Config, Diff, FieldPathMode};

use crate::{
    serialization::{ModificationSerializer, SerializationStrategy},
    Identifier, ModificationEvent, TrackableMarker,
};

/// Tracks value modifications of a type and sends events with these changes.
///
/// The [Tracker](./struct.Tracker.html) implements [DerefMut](./struct.Tracker.html#impl-DerefMut) which makes it possible to treat this tracker as if you are working with the type you track.
/// On [Drop](./struct.Tracker.html#impl-Drop) it checks if modifications have been made.
/// If this is the case only the modified fields in an event will be sent to the given sender.
pub struct Tracker<'borrow, 'notifier, C, S, I>
where
    C: TrackableMarker,
    S: SerializationStrategy,
    I: Identifier,
{
    old_copy: C,
    borrow: &'borrow mut C,
    notifier: &'notifier Sender<ModificationEvent<I>>,
    serialization: S,
    identifier: I,
}

impl<'borrow, 'notifier, C, S, I> Tracker<'borrow, 'notifier, C, S, I>
where
    C: TrackableMarker,
    S: SerializationStrategy,
    I: Identifier,
{
    /// Constructs a new tracker.
    ///
    /// * `borrow`: mutable reference to the object which modifications are tracked.
    /// * `notifier`: a sender where mutation events are sent.
    /// * `serialization`: an instance of a type that implements [SerializationStrategy](../track/serialization/trait.SerializationStrategy.html) strategy.
    ///     This serializer is needed to monitor the changes and the serialized mutations are sent along with the event.
    /// * `identifier`: An identifier with which you can relate the modification event to your type.
    /// The identifier should implement [Identifier](../track/trait.Identifier.html)
    pub fn new(
        borrow: &'borrow mut C,
        notifier: &'notifier Sender<ModificationEvent<I>>,
        serialization: S,
        identifier: I,
    ) -> Tracker<'borrow, 'notifier, C, S, I> {
        Tracker {
            old_copy: (borrow.deref()).clone(),
            borrow,
            notifier,
            identifier,
            serialization,
        }
    }
}

impl<'borrow, 'notifier, C, S, I> Deref for Tracker<'borrow, 'notifier, C, S, I>
where
    C: TrackableMarker,
    S: SerializationStrategy,
    I: Identifier,
{
    type Target = C;

    /// Returns a reference to the underlying type being tracked.
    fn deref(&self) -> &Self::Target {
        &self.borrow
    }
}

impl<'borrow, 'notifier, C, S, I> DerefMut for Tracker<'borrow, 'notifier, C, S, I>
where
    C: TrackableMarker,
    S: SerializationStrategy,
    I: Identifier,
{
    /// Returns a mutable reference to the underlying type being tracked.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.borrow
    }
}

impl<'borrow, 'notifier, C, S, I> Drop for Tracker<'borrow, 'notifier, C, S, I>
where
    C: TrackableMarker,
    S: SerializationStrategy,
    I: Identifier,
{
    /// Checks to see if any field values have changed.
    /// If this is the case, the changed fields will be packed into an event and an event will be sent.
    fn drop(&mut self) {
        let diff = Config::new()
            .with_field_path_mode(FieldPathMode::Index)
            .serializable_diff(&self.old_copy, &self.borrow);

        let serializer = ModificationSerializer::new(self.serialization.clone());

        match serializer.serialize::<Diff<C>>(&diff) {
            Ok(data) => {
                if diff.has_changes() {
                    self.notifier
                        .send(ModificationEvent::new(data, self.identifier))
                        .expect("The sender for modification events panicked. Is the receiver still alive?");
                }
            }
            Err(e) => {
                panic!(
                    "Could not serialize modification information because: {:?}",
                    e
                );
            }
        };
    }
}
