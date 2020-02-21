use crate::Identifier;

#[derive(Clone, Debug)]
/// A modification event.
pub struct ModificationEvent<I: Identifier> {
    /// The serialized data of the modified structure fields.
    pub modified_fields: Vec<u8>,
    /// The reference to the corresponding type.
    pub identifier: I,
}

impl<I: Identifier> ModificationEvent<I> {
    /// Constructs a new [Modification Event](struct.ModificationEvent.html).
    pub fn new(data: Vec<u8>, identifier: I) -> Self {
        ModificationEvent {
            modified_fields: data,
            identifier,
        }
    }
}
