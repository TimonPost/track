use criterion::{criterion_group, Criterion};
use crossbeam_channel::unbounded;
use track::preclude::*;

#[track]
#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

pub fn make_change_and_monitor(notifier: &Sender<ModificationEvent>, serializable: &mut Position) {
    let mut component = serializable.track(&notifier);
    component.x += 0.4;
    component.y += 0.3;
}

pub fn monitoring_changes_benchmark(c: &mut Criterion) {
    c.bench_function("Monitoring Change", |b| {
        let (tx, rx) = unbounded();
        let mut position = Position { x: 32.5, y: 11.3 };

        b.iter(|| make_change_and_monitor(&tx, &mut position));
    });
}

criterion_group!(monitoring, monitoring_changes_benchmark);
