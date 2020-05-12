use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::event::ModificationEvent;

/// An event channel over which modification events are sent.
pub struct ModificationChannel<I: Copy + Clone + Send + Sync> {
    event_receiver: Receiver<ModificationEvent<I>>,
    event_sender: Sender<ModificationEvent<I>>,
}

impl<I: Copy + Clone + Send + Sync> ModificationChannel<I> {
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
