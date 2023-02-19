use criterion::{criterion_group, criterion_main, Criterion};

use union_find::Repr;

fn sum_bench(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("amortized_unions / 10_000 elements", |b| {
        b.iter(|| {
            let mut root = Repr::new(-1);

            let mut _nodes = Vec::new();

            for k in 0..10_000 {
                let mut node = Repr::new(k);
                root.union(&mut node);
                _nodes.push(node);
            }
        })
    });
    c
}

criterion_group!(benches, sum_bench);
criterion_main!(benches);
