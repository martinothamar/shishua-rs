#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

use criterion::measurement::Measurement;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use criterion_perf_events::Perf;
use perfcnt::linux::HardwareEventType as Hardware;
use perfcnt::linux::PerfCounterBuilderLinux as Builder;
use rand::Rng;
use rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use simd_rand::specific::avx2::{*, SimdPrng};
use simd_rand::specific::avx512::{*, SimdPrng as SimdPrngX8};

const ITERATIONS: usize = 16;

#[inline(always)]
fn do_xoshiro_u64(rng: &mut Xoshiro256Plus, data: &mut U64x4) {
    for _ in 0..ITERATIONS {
        let data = black_box(&mut *data);
        data[0] = rng.next_u64();
        data[1] = rng.next_u64();
        data[2] = rng.next_u64();
        data[3] = rng.next_u64();
    }
}
#[inline(always)]
fn do_xoshiro_x4_u64(rng: &mut Xoshiro256PlusX4, data: &mut U64x4) {
    for _ in 0..ITERATIONS {
        rng.next_u64x4(black_box(data));
    }
}

#[inline(always)]
fn do_xoshiro_f64(rng: &mut Xoshiro256Plus, data: &mut F64x4) {
    for _ in 0..ITERATIONS {
        let data = black_box(&mut *data);
        data[0] = rng.gen_range(0.0..1.0);
        data[1] = rng.gen_range(0.0..1.0);
        data[2] = rng.gen_range(0.0..1.0);
        data[3] = rng.gen_range(0.0..1.0);
    }
}
#[inline(always)]
fn do_xoshiro_x4_f64(rng: &mut Xoshiro256PlusX4, data: &mut F64x4) {
    for _ in 0..ITERATIONS {
        rng.next_f64x4(black_box(data));
    }
}
#[inline(always)]
fn do_xoshiro_x8_f64(rng: &mut Xoshiro256PlusX8, data: &mut F64x8) {
    for _ in 0..(ITERATIONS / 2) {
        rng.next_f64x8(black_box(data));
    }
}

fn bench<M: Measurement, const T: u8>(c: &mut Criterion<M>) {
    let mut group = c.benchmark_group("xoshiro256plus");

    group.throughput(Throughput::Bytes((ITERATIONS * mem::size_of::<U64x4>()) as u64));

    let suffix = match T {
        Type::TIME => "Time",
        Type::INST => "Instructions",
        Type::CYCLES => "Cycles",
        _ => unreachable!(),
    };

    let xoshiro_u64_name = format!("Xoshiro256Plus u64x4 - {suffix}");
    let xoshiro_x4_u64_name = format!("Xoshiro256PlusX4 u64x4 - {suffix}");
    let xoshiro_f64_name = format!("Xoshiro256Plus f64x4 - {suffix}");
    let xoshiro_x4_f64_name = format!("Xoshiro256PlusX4 f64x4 - {suffix}");
    let xoshiro_x8_f64_name = format!("Xoshiro256PlusX8 f64x8 - {suffix}");

    group.bench_function(xoshiro_u64_name, |b| {
        let mut rng: Xoshiro256Plus = Xoshiro256Plus::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
        let mut data: U64x4 = Default::default();

        b.iter(|| do_xoshiro_u64(&mut rng, black_box(&mut data)))
    });
    group.bench_function(xoshiro_x4_u64_name, |b| {
        let mut rng: Xoshiro256PlusX4 = Xoshiro256PlusX4::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
        let mut data: U64x4 = Default::default();

        b.iter(|| do_xoshiro_x4_u64(&mut rng, black_box(&mut data)))
    });

    group.bench_function(xoshiro_f64_name, |b| {
        let mut rng: Xoshiro256Plus = Xoshiro256Plus::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
        let mut data: F64x4 = Default::default();

        b.iter(|| do_xoshiro_f64(&mut rng, black_box(&mut data)))
    });
    group.bench_function(xoshiro_x4_f64_name, |b| {
        let mut rng: Xoshiro256PlusX4 = Xoshiro256PlusX4::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
        let mut data: F64x4 = Default::default();

        b.iter(|| do_xoshiro_x4_f64(&mut rng, black_box(&mut data)))
    });
    group.bench_function(xoshiro_x8_f64_name, |b| {
        let mut rng: Xoshiro256PlusX8 = Xoshiro256PlusX8::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
        let mut data: F64x8 = Default::default();

        b.iter(|| do_xoshiro_x8_f64(&mut rng, black_box(&mut data)))
    });

    group.finish();
}

#[non_exhaustive]
struct Type;

impl Type {
    pub const TIME: u8 = 1;
    pub const INST: u8 = 2;
    pub const CYCLES: u8 = 3;
}

criterion_group!(
    name = time;
    config = Criterion::default();
    targets = bench::<_, { Type::TIME }>
);
// criterion_group!(
//     name = instructions;
//     config = Criterion::default().with_measurement(Perf::new(Builder::from_hardware_event(Hardware::Instructions)));
//     targets = bench::<_, { Type::INST }>
// );
// criterion_group!(
//     name = cycles;
//     config = Criterion::default().with_measurement(Perf::new(Builder::from_hardware_event(Hardware::RefCPUCycles)));
//     targets = bench::<_, { Type::CYCLES }>
// );
criterion_main!(time);
