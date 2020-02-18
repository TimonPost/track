use std::ops::{Deref, DerefMut};

use crossbeam_channel::Sender;
use serde_diff::{Config, Diff, FieldPathMode};
use uuid::Uuid;

use crate::{
    serialisation::{ModificationSerializer, SerialisationStrategy},
    ModificationEvent, TrackableMarker,
};

/// Tracks value modifications of a type and sends events with these changes.
///
/// The [Tracker](LINK) implements [Deref](LINK) which makes it possible to treat this tracker as if you are working with the type you track.
/// On [Drop](LINK) it checks if modifications have been made.
/// If this is the case only the modified fields in an event will be sent to the given sender.
pub struct Tracker<'borrow, 'notifier, C, S>
where
    C: TrackableMarker,
    S: SerialisationStrategy,
{
    old_copy: C,
    borrow: &'borrow mut C,
    notifier: &'notifier Sender<ModificationEvent>,
    serialisation: S,
    identifier: Option<Uuid>,
}

impl<'borrow, 'notifier, C, S> Tracker<'borrow, 'notifier, C, S>
where
    C: TrackableMarker,
    S: SerialisationStrategy,
{
    /// Constructs a new tracker.
    ///
    /// * `borrow`: mutable reference to the object which modifications are tracked.
    /// * `notifier`: a sender where mutation events are sent.
    /// * `serialisation`: an instance of a type that implements [SerialisationStrategy](LINK) strategy.
    ///     This serializer is needed to monitor the changes and the serialized mutations are sent along with the event.
    /// * `identifier`: Optionally you can give an identifier with which you can link the event to the type.
    pub fn new(
        borrow: &'borrow mut C,
        notifier: &'notifier Sender<ModificationEvent>,
        serialisation: S,
        identifier: Option<Uuid>,
    ) -> Tracker<'borrow, 'notifier, C, S> {
        Tracker {
            old_copy: (borrow.deref()).clone(),
            borrow,
            notifier,
            identifier,
            serialisation,
        }
    }
}

impl<'borrow, 'notifier, C, S> Deref for Tracker<'borrow, 'notifier, C, S>
where
    C: TrackableMarker,
    S: SerialisationStrategy,
{
    type Target = C;

    /// Returns a reference to the underlying type being tracked.
    fn deref(&self) -> &Self::Target {
        &self.borrow
    }
}

impl<'borrow, 'notifier, C, S> DerefMut for Tracker<'borrow, 'notifier, C, S>
where
    C: TrackableMarker,
    S: SerialisationStrategy,
{
    /// Returns a mutable reference to the underlying type being tracked.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.borrow
    }
}

impl<'borrow, 'notifier, C, S> Drop for Tracker<'borrow, 'notifier, C, S>
where
    C: TrackableMarker,
    S: SerialisationStrategy,
{
    /// Checks to see if any field values have changed.
    /// If this is the case, the changed fields will be packed into an event and an event will be sent.
    fn drop(&mut self) {
        let diff = Config::new()
            .with_field_path_mode(FieldPathMode::Index)
            .serializable_diff(&self.old_copy, &self.borrow);

        let serializer = ModificationSerializer::new(self.serialisation.clone());

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
