use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use recursive_multiply::recursive_mul2;

pub fn bench_recursive_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("recursive_mul2");
    [(11, 13), (10_000, 10_000), (123456, 123456)]
        .iter()
        .for_each(|&(a, b): &(u64, u64)| {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:?}", (a, b))),
                &(a, b),
                |bb, &(a, b)| bb.iter(|| recursive_mul2(a, b)),
            );
        });
}

criterion_group!(benches, bench_recursive_mul);
criterion_main!(benches);
