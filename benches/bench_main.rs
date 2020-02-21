use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::serialization::serialization,
    benchmarks::change_monitoring::monitoring,
}
