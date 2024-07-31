use criterion::{criterion_group, Criterion};
use crate::old_granular::Granulator as OldGranulator;
use rust_dsp::{
  buffer::Buffer, envelope::{BreakPoints, EnvType, Envelope}, grains::Granulator as NewGranulator, interpolation::Linear, waveshape::traits::Waveshape
};

use rand::{thread_rng, Rng};




fn grain_old(og: &mut OldGranulator<32, 240000>) -> f32 {
  og.play::<Linear, Linear>(0.5, 0.5, 1.0,  1.0)

}
fn grain_new(ng: &mut NewGranulator<32, 240000>) -> f32 {
  ng.play::<Linear, Linear>(0.5, 0.5, 1.0, 0.1, 1.0)
}

pub fn criterion_benchmark_grains(c: &mut Criterion) {
  const SIZE: usize = 1<<13;
  const BUFSIZE: usize = 48000*5;

  let shape: EnvType<0, 0> = EnvType::Vector([0.0;SIZE].hanning().to_vec());
  let mut og = OldGranulator::<32, BUFSIZE>::new(
    Buffer::new(48000.0),
    Envelope::new(&shape, 48000.0),
    48000.0
  );

  let shape: EnvType<0, 0> = EnvType::Vector([0.0;SIZE].hanning().to_vec());
  let mut g = NewGranulator::<32, BUFSIZE>::new(&shape, 48000.0);

  while og.record(thread_rng().gen_range(0.0..1.0)).is_some() {continue;}
  while g.record(thread_rng().gen_range(0.0..1.0)).is_some() {continue;}


  let mut group = c.benchmark_group("grains");

  group.bench_function("og grains",
    |b| 
    b.iter(|| {grain_old(&mut og)}
  ));
  group.bench_function("ng grains", 
    |b| b.iter(|| {grain_new(&mut g)}));






}
