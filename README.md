[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=Z8QK6XU749JB2) 
[![Latest Version][crate-badge]][crate-link] 
[![docs][docs-badge]][docs-link]
![Lines of Code][loc-badge]
[![MIT][license-badge]][license-link] 

# Track Data Modifications
This library offers a boilerplate free approach to track struct data modifications. 
For optimization, only the adjusted fields are tracked. Changes will be serialized and sent on an channel.

## Features

- [X] Monitoring modifications in data
- [X] Serde based (excluding fields, ...)
- [X] Applying modifications to a type
- [x] Customizable Serialization


## Feature Flags

| Feature | Description |
| :----- | :----- |
| `bincode-serialization` | serialization using [bincode](https://crates.io/crates/bincode) (enabled by default) .|
| `rmp-serialization` | serialization using [rmp-serde](https://crates.io/crates/rmp-serde) .|

_Optionally implement your own serializer met SerializationStrategy](track/serialization/trait.SerializationStrategy.html)._

## Examples

First, add `track` attribute to mark your struct as trackable.
```rust
// imports all necessarily types for the `track` attribute.
use track::preclude::*;

#[track]
#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
```

You can specify a serialization method for the track macro.
Give the name of the type that implements [SerializationStrategy](https://docs.rs/track/serialization/trait.SerializationStrategy.html), and make sure it is in scope for the macro. 
Such as: 

```rust
use track::serialization::bincode::Bincode;

#[track(serialization = "Bincode")]
struct Postition ...
```

Now let us make some modifications and apply those to other instances.
```rust
use track::{preclude::*, serialization::bincode::Bincode, Apply, ModificationChannel};

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
```

_For a more in-depth example checkout the [examples](https://github.com/entity-sync-rs/track/tree/master/examples) on github._

[crate-badge]: https://img.shields.io/crates/v/track.svg
[crate-link]: https://crates.io/crates/track

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: ./docs/LICENSE

[docs-badge]: https://docs.rs/track/badge.svg
[docs-link]: https://docs.rs/track/

[loc-badge]: https://tokei.rs/b1/github/entity-sync-rs/track?category=code