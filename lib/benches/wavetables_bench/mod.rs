use alloc::sync::Arc;

use criterion::Criterion;

use rust_dsp::{
  wavetable::{
    shared::Wavetable as ShareTable,
    owned::Wavetable as OwnTable,
  }, waveshape::traits::Waveshape,
  interpolation::Linear,
};

const BLOCK_SIZE: usize = 1 << 16;
    
#[cfg(feature="std")]
use rust_dsp::wavetable::arc::Wavetable as ArcTable;
use std::sync::RwLock;


fn run_table<const N: usize>(wt: &mut ShareTable, &table: &[f32; N]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Linear>(&table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

#[cfg(feature="std")]
fn run_table_arc(wt: &mut ArcTable) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Linear>(freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

#[cfg(feature="std")]
fn run_table_own<const N: usize>(wt: &mut OwnTable<N>) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Linear>(freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_table_mod<const N: usize>(wt: &mut ShareTable, &table: &[f32; N], lfo: &mut ShareTable) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  let mut m = 0.0;
  for i in 0..BLOCK_SIZE {
    m = if i % 8 == 0 { lfo.play::<Linear>(&table, 2.0, 0.0) } else { m };
    out = wt.play::<Linear>(&table, freq, m);
    freq += 0.2351;
  }
  out
}

fn run_table_own_mod<const N: usize>(wt: &mut OwnTable<N>, lfo: &mut OwnTable<N>) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  let mut m = 0.0;
  for i in 0..BLOCK_SIZE {
    m = if i % 8 == 0 { lfo.play::<Linear>(2.0, 0.0) } else { m };
    out = wt.play::<Linear>(freq, m);
    freq += 0.2351;
  }
  out
}

#[cfg(feature="std")]
fn run_table_arc_mod(wt: &mut ArcTable, lfo: &mut ArcTable) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  let mut m = 0.0;
  for i in 0..BLOCK_SIZE {
    m = if i % 8 == 0 { lfo.play::<Linear>(2.0, 0.0) } else { m };
    out = wt.play::<Linear>(freq, m);
    freq += 0.2351;
  }
  out
}


pub fn criterion_benchmark_tables(c: &mut Criterion) {
  const SIZE: usize = 1<<15;
  let table = [0.0; SIZE].sine();
  let mut wt = ShareTable::new();
  wt.set_samplerate(48000);
  let mut group = c.benchmark_group("tables");

  let otable = [0.0;SIZE].sine();
  let mut owt = OwnTable::new(&otable, 48000);

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
    ArcTable::new(atable, 48000) 
  };
  group.bench_function(
    "wt arc",
    |b| b.iter(|| {run_table_arc(&mut awt)}) 
  );

  drop(group);
  let mut lfo = ShareTable::new();
  lfo.set_samplerate(48000);
  let mut olfo = OwnTable::new(&otable, 48000);
  #[cfg(feature="std")]
  let mut  alfo = { 
    let atable = Arc::new(RwLock::new([0.0;SIZE].sine().to_vec()));
    ArcTable::new(atable, 48000) };

  let mut group_mod = c.benchmark_group("tables_mod");
  group_mod.bench_function(
    "wt owned mod",
    |b| b.iter(|| {run_table_own_mod(&mut owt, &mut olfo)}) 
  );
  
  group_mod.bench_function(
    "wt shared mod",
    |b| b.iter(|| {run_table_mod(&mut wt, &table, &mut lfo)}) 
  );
  
  group_mod.bench_function(
    "wt arc mod",
    |b| b.iter(|| {run_table_arc_mod(&mut awt, &mut alfo)}) 
  );
}
