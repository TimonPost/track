use crate::preclude::Uuid;

#[derive(Clone, Debug)]
/// A modification event.
pub struct ModificationEvent {
    /// The serialized data of the modified structure fields.
    pub modified_fields: Vec<u8>,
    /// The reference to the corresponding type.
    pub identifier: Option<Uuid>,
}

impl ModificationEvent {
    /// Constructs a new [Modification Event](LINK).
    pub fn new(data: Vec<u8>, identifier: Option<Uuid>) -> Self {
        ModificationEvent {
            modified_fields: data,
            identifier,
        }
    }
}
