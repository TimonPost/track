use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::serialisation::serialisation,
    benchmarks::change_monitoring::monitoring,
}
