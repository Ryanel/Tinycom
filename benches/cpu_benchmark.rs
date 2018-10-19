#[macro_use]
extern crate criterion;
extern crate tinycom_common;

use criterion::Criterion;
use tinycom_common::cpu::*;

fn register_benchmark(c: &mut Criterion) {
    //
    c.bench_function("reg add 1", |b| {
        let mut reg : Register<u32> = Register {value: 0};
        b.iter(|| criterion::black_box(reg.add(1)))
    });

    c.bench_function("reg checked add 1", |b| {
        let mut reg : Register<u32> = Register {value: 0};
        b.iter(|| criterion::black_box(reg.add_checked(1)))
    });

    c.bench_function("reg checked add 0xFFFF_FFFF", |b| {
        let mut reg : Register<u32> = Register {value: 0};
        b.iter(|| criterion::black_box(reg.add_checked(0xFFFF_FFFF)))
    });
}

criterion_group!(benches, register_benchmark);
criterion_main!(benches);