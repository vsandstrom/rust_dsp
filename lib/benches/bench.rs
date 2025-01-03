extern crate alloc;

mod wavetables_bench;
mod grain_bench;
mod midi_input_bench;
mod delay_bench;
// mod poly_bench;
// mod old_poly;
mod old_granular;
use criterion::{criterion_group, criterion_main};

use wavetables_bench::criterion_benchmark_tables;
use grain_bench::criterion_benchmark_grains;
use midi_input_bench::criterion_benchmark_midi;
use delay_bench::criterion_benchmark_delay;


criterion_group!(
  benches,
  criterion_benchmark_tables,
  criterion_benchmark_grains,
  criterion_benchmark_midi,
  criterion_benchmark_delay
);
criterion_main!(benches);
