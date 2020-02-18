use track::{preclude::*, serialisation::bincode::Bincode, Apply, ModificationChannel};

#[track(serialisation = "Bincode")]
#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Identity {
    pub value: u8,
}

impl Identifier for Identity {}

fn main() {
    let channel = ModificationChannel::<Identity>::new();

    let updated_storage = vec![
        (Identity { value: 1 }, Position { x: 0, y: 0 }),
        (Identity { value: 2 }, Position { x: 0, y: 0 }),
    ];
    let mut outdated_storage = updated_storage.clone();

    // == Make changes to storage ==
    make_changes(&channel, updated_storage);

    // == Apply changes to outdated storage ==
    apply_changes(&channel, &mut outdated_storage);
}

fn make_changes(channel: &ModificationChannel<Identity>, entities: Vec<(Identity, Position)>) {
    for (id, mut position) in entities {
        let mut position = position.track(channel.sender(), id); /* returns `Tracker` which tracks changes */

        // `Tracker` implements `DerefMut`
        position.x += 1;
        position.y += 1;
    } // <- on the `Drop` of `wrapper` changes are serialized and sent on the channel.
}

fn apply_changes(
    channel: &ModificationChannel<Identity>,
    entities: &mut Vec<(Identity, Position)>,
) {
    for event in channel.receiver().try_iter() {
        let entity = entities
            .iter_mut()
            .find(|e| e.0 == event.identifier)
            .unwrap();

        Apply::apply_to(&mut entity.1, &event.modified_fields, Bincode);

        println!("entity updated {:?}", entity);
    }
}
