#[derive(Clone, Debug)]
/// A modification event.
pub struct ModificationEvent<I> {
    /// The serialized data of the modified structure fields.
    pub modified_fields: Vec<u8>,
    /// The reference to the corresponding type.
    pub identifier: I,
}

impl<I> ModificationEvent<I> {
    /// Constructs a new [Modification Event](LINK).
    pub fn new(data: Vec<u8>, identifier: I) -> Self {
        ModificationEvent {
            modified_fields: data,
            identifier,
        }
    }
}
