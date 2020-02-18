use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::event::ModificationEvent;
use crate::Identifier;

/// An event channel over which modification events are sent.
pub struct ModificationChannel<I: Identifier> {
    event_receiver: Receiver<ModificationEvent<I>>,
    event_sender: Sender<ModificationEvent<I>>,
}

impl<I: Identifier> ModificationChannel<I> {
    /// Constructs a new modification channel.
    pub fn new() -> ModificationChannel<I> {
        let (tx, rx) = unbounded();

        ModificationChannel {
            event_receiver: rx,
            event_sender: tx,
        }
    }

    /// Returns an sender on which modification events are sent.
    pub fn sender(&self) -> &Sender<ModificationEvent<I>> {
        &self.event_sender
    }

    /// Returns a receiver on which modification events can be received.
    pub fn receiver(&self) -> &Receiver<ModificationEvent<I>> {
        &self.event_receiver
    }
}
