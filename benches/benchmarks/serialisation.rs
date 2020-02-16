use criterion::{criterion_group, Criterion};
use serde::Serialize;

use track::serialisation::bincode::Bincode;
use track::serialisation::rmp::Rmp;
use track::serialisation::{ModificationSerializer, SerialisationStrategy};

#[derive(Clone, Serialize)]
struct Position {
    x: f32,
    y: f32,
}

fn serialize<T: SerialisationStrategy, C: Serialize>(
    serializer: &ModificationSerializer<T>,
    serializable: C,
) {
    serializer.serialize(&serializable);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Serialisation with bincode", |b| {
        let serializer = ModificationSerializer::new(Bincode);
        let packet = Position { x: 12.5, y: 33.6 };

        b.iter(|| serialize::<Bincode, Position>(&serializer, packet.clone()));
    });

    c.bench_function("Serialisation with rmp-serde", |b| {
        let serializer = ModificationSerializer::new(Rmp);
        let packet = Position { x: 12.5, y: 33.6 };

        b.iter(|| serialize::<Rmp, Position>(&serializer, packet.clone()));
    });
}

criterion_group!(serialisation, criterion_benchmark);
