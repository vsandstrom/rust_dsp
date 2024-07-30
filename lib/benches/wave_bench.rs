mod old_wavetables;
mod old_granular;
use criterion::{criterion_group, criterion_main, Criterion};
  use std::sync::{Arc, RwLock};
use rust_dsp::{
  wavetable::WaveTable,
  waveshape::traits::Waveshape,
  interpolation::Linear,
};

use old_wavetables::{OwnWaveTable, ArcWaveTable};

fn run_table<const N: usize>(wt: &mut WaveTable, &table: &[f32; N]) -> f32 {
  wt.play::<N, Linear>(&table, 100.0, 0.0)
}

fn run_table_arc(wt: &mut ArcWaveTable) -> f32 {
  wt.play::<Linear>(100.0, 0.0)
}

fn run_table_own<const N: usize>(wt: &mut OwnWaveTable<N>) -> f32 {
  wt.play::<Linear>(100.0, 0.0)
}

fn criterion_benchmark(c: &mut Criterion) {

  // let table = [0.0; 1<<5].sine();
  // let mut wt = WaveTable::from(48000.0);
  
  let atable = Arc::new(RwLock::new([0.0;1<<5].sine().to_vec()));
  let mut awt = ArcWaveTable::new(atable, 48000.0);

  let otable = [0.0;1<<5].sine();
  let mut owt = OwnWaveTable::new(&otable, 48000.0);

  let mut group = c.benchmark_group("tables!!");

  group.bench_function(
    "wt opt",
    |b|
      b.iter(|| {
        run_table(&mut WaveTable::from(48000.0), &[0.0; 1<<5].sine())
      }
    ) 
  );

  group.bench_function(
    "wt arc",
    |b|
      b.iter(|| {
        run_table_arc(&mut awt)
      }
    ) 
  );
  
  group.bench_function(
    "wt own",
    |b|
      b.iter(|| {
        run_table_own(&mut owt)
      }
    ) 
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



