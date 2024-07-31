mod wavetables_bench;
mod grain_bench;
mod old_granular;
use criterion::{criterion_group, criterion_main};

use wavetables_bench::criterion_benchmark_tables;
use grain_bench::criterion_benchmark_grains;


criterion_group!(benches, criterion_benchmark_tables, criterion_benchmark_grains);
criterion_main!(benches);
