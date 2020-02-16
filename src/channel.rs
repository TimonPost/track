use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::event::ModificationEvent;

/// An event channel over which modification events are sent.
pub struct ModificationChannel {
    event_receiver: Receiver<ModificationEvent>,
    event_sender: Sender<ModificationEvent>,
}

impl ModificationChannel {
    /// Constructs a new modification channel.
    pub fn new() -> ModificationChannel {
        let (tx, rx) = unbounded();

        ModificationChannel {
            event_receiver: rx,
            event_sender: tx,
        }
    }

    /// Returns an sender on which modification events are sent.
    pub fn sender(&self) -> &Sender<ModificationEvent> {
        &self.event_sender
    }

    /// Returns a receiver on which modification events can be received.
    pub fn receiver(&self) -> &Receiver<ModificationEvent> {
        &self.event_receiver
    }
}
