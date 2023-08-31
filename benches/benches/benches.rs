use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use unicode_validation_example::{v1, v2};

mod data;

use data::*;

/// прогон бенчмарков валидации
fn validation(c: &mut Criterion)
{
    let mut group = c.benchmark_group("validation");
    group.measurement_time(core::time::Duration::from_secs(5));

    for data in validation_data_files() {
        // эталонная функция - встроенная core::str::from_utf8
        group.bench_with_input(
            BenchmarkId::new("core", &data.name),
            data.source.as_slice(),
            |b, data| b.iter(|| core::str::from_utf8(data)),
        );

        // замеряем наши варианты валидации

        // v1
        group.bench_with_input(
            BenchmarkId::new("v1", &data.name),
            data.source.as_slice(),
            |b, data| b.iter(|| v1::from_utf8(data)),
        );

        // v2
        group.bench_with_input(
            BenchmarkId::new("v2", &data.name),
            data.source.as_slice(),
            |b, data| b.iter(|| v2::from_utf8(data)),
        );
    }

    group.finish();
}

criterion_group!(benches, validation);
criterion_main!(benches);
