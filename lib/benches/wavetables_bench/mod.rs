use std::sync::{Arc, RwLock};

use criterion::Criterion;

use rust_dsp::{
  wavetable::{
    shared::WaveTable as ShareTable,
    owned::WaveTable as OwnTable,
    arc::WaveTable as ArcTable,
  }, waveshape::traits::Waveshape,
  interpolation::Linear,
};


fn run_table<const N: usize>(wt: &mut ShareTable, &table: &[f32; N]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..256 {
    out = wt.play::<N, Linear>(&table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_table_arc(wt: &mut ArcTable) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..256 {
    out = wt.play::<Linear>(freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_table_own<const N: usize>(wt: &mut OwnTable<N>) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..256 {
    out = wt.play::<Linear>(freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

pub fn criterion_benchmark_tables(c: &mut Criterion) {
  const SIZE: usize = 1<<13;
  let table = [0.0; SIZE].sine();
  let mut wt = ShareTable::new();
  wt.set_samplerate(48000.0);
  
  let atable = Arc::new(RwLock::new([0.0;SIZE].sine().to_vec()));
  let mut awt = ArcTable::new(atable, 48000.0);

  let otable = [0.0;SIZE].sine();
  let mut owt = OwnTable::new(&otable, 48000.0);

  let mut group = c.benchmark_group("tables");

  group.bench_function(
    "wt shared",
    |b| b.iter(|| {run_table(&mut wt, &table)}) 
  );

  group.bench_function(
    "wt arc",
    |b| b.iter(|| {run_table_arc(&mut awt)}) 
  );
  
  group.bench_function(
    "wt owned",
    |b| b.iter(|| {run_table_own(&mut owt)}) 
  );
}
