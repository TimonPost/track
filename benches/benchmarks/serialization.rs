use criterion::{criterion_group, Criterion};
use serde::Serialize;

use track::serialization::bincode::Bincode;
use track::serialization::rmp::Rmp;
use track::serialization::{ModificationSerializer, SerializationStrategy};

#[derive(Clone, Serialize)]
struct Position {
    x: f32,
    y: f32,
}

fn serialize<T: SerializationStrategy, C: Serialize>(
    serializer: &ModificationSerializer<T>,
    serializable: C,
) {
    serializer.serialize(&serializable);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Serialization with bincode", |b| {
        let serializer = ModificationSerializer::new(Bincode);
        let packet = Position { x: 12.5, y: 33.6 };

        b.iter(|| serialize::<Bincode, Position>(&serializer, packet.clone()));
    });

    c.bench_function("Serialization with rmp-serde", |b| {
        let serializer = ModificationSerializer::new(Rmp);
        let packet = Position { x: 12.5, y: 33.6 };

        b.iter(|| serialize::<Rmp, Position>(&serializer, packet.clone()));
    });
}

criterion_group!(serialization, criterion_benchmark);
