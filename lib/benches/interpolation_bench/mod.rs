use criterion::Criterion;
use rust_dsp::{
  wavetable::{
    shared::Wavetable,
  }, waveshape::traits::Waveshape,
  interpolation::{Linear, Cubic, Hermite, Floor},
  delay::{Delay, FixedDelay, delay}
};

const BLOCK_SIZE: usize = 1 <<16;


fn run_linear(wt: &mut Wavetable, table: &[f32]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Linear>(table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_cubic(wt: &mut Wavetable, table: &[f32]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Cubic>(table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_hermite(wt: &mut Wavetable, table: &[f32]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Hermite>(table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_floor(wt: &mut Wavetable, table: &[f32]) -> f32 {
  let mut out = 0.0;
  let mut freq = 100.0;
  for i in 0..BLOCK_SIZE {
    out = wt.play::<Floor>(table, freq, 0.0);
    if i % 64 == 0 { freq += 10.0; }
  }
  out
}

fn run_delay_floor(d: &mut Delay, input: f32, buffer: &mut [f32]) {
  for _ in 0..BLOCK_SIZE {d.play::<Floor>(buffer, input, 0.1, 0.1);}
}

fn run_delay_linear(d: &mut Delay, input: f32, buffer: &mut [f32]) {
  for _ in 0..BLOCK_SIZE {d.play::<Linear>(buffer, input, 0.1, 0.1);}
}

fn run_delay_cubic(d: &mut Delay, input: f32, buffer: &mut [f32]) {
  for _ in 0..BLOCK_SIZE {d.play::<Cubic>(buffer, input, 0.1, 0.1);}
}

fn run_delay_hermite(d: &mut Delay, input: f32, buffer: &mut [f32]) {
  for _ in 0..BLOCK_SIZE {d.play::<Hermite>(buffer, input, 0.1, 0.1);}
}

pub fn criterion_benchmark_interpolation(c: &mut Criterion) {
  let mut wt = Wavetable::new();
  let table = [0.0; 1<<13].sine();
  wt.set_samplerate(48000);

  let mut group = c.benchmark_group("interpolation_table");

  group.bench_function(
    "table floor",
    |b| b.iter(|| {run_floor(&mut wt, &table)}) 
  );

  group.bench_function(
    "table linear",
    |b| b.iter(|| {run_linear(&mut wt, &table)}) 
  );

  group.bench_function(
    "table cubic",
    |b| b.iter(|| {run_cubic(&mut wt, &table)}) 
  );

  group.bench_function(
    "table hermite",
    |b| b.iter(|| {run_hermite(&mut wt, &table)}) 
  );

  drop(group);

  let mut d = Delay::new();
  let mut group = c.benchmark_group("interpolation_delay");
  let mut buffer = [0.0; {48000*8}];

  let signal = 1.0;
  group.bench_function("delay floor", |b| {
    b.iter(|| {
      run_delay_floor(&mut d, signal, &mut buffer);
    })
  });
  group.bench_function("delay linear", |b| {
    b.iter(|| {
      run_delay_linear(&mut d, signal, &mut buffer);
    })
  });
  group.bench_function("delay cubic", |b| {
    b.iter(|| {
      run_delay_cubic(&mut d, signal, &mut buffer);
    })
  });
  group.bench_function("delay hermite", |b| {
    b.iter(|| {
      run_delay_hermite(&mut d, signal, &mut buffer);
    })
  });

}


