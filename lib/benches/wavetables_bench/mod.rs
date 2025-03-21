use alloc::sync::Arc;

use criterion::Criterion;

use rust_dsp::{
  wavetable::{
    shared::Wavetable as ShareTable,
    owned::Wavetable as OwnTable,

  }, waveshape::traits::Waveshape,
  interpolation::Linear,
};
    
#[cfg(feature="std")]
use rust_dsp::wavetable::arc::Wavetable as ArcTable;
use std::sync::RwLock;


fn run_table<const N: usize>(wt: &mut ShareTable, &table: &[f32; N]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..256 {
    out = wt.play::<Linear>(&table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}


#[cfg(feature="std")]
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
  let mut group = c.benchmark_group("tables");

  let otable = [0.0;SIZE].sine();
  let mut owt = OwnTable::new(&otable, 48000.0);

  group.bench_function(
    "wt owned",
    |b| b.iter(|| {run_table_own(&mut owt)}) 
  );

  group.bench_function(
    "wt shared",
    |b| b.iter(|| {run_table(&mut wt, &table)}) 
  );

  #[cfg(feature="std")]
  let mut  awt = {
    let atable = Arc::new(RwLock::new([0.0;SIZE].sine().to_vec()));
    ArcTable::new(atable, 48000.0)
  };
  group.bench_function(
    "wt arc",
    |b| b.iter(|| {run_table_arc(&mut awt)}) 
  );
  
}
