use criterion::Criterion;
use rust_dsp::delay::{Delay, FixedDelay, delay};
use rust_dsp::interpolation::Linear;

const SIZE: usize = 8*48000;
const DELAY: usize = 12345;

fn delay_interpolated(d: &mut Delay, input: f32, buffer: &mut [f32]) {
  for _ in 0..256 {d.play::<Linear>(buffer, input, 0.1, 0.1);}
}

fn fixed_delay(fd: &mut FixedDelay<DELAY, SIZE>, input: f32) {
  for _ in 0..256 {fd.play(input, 0.1);}
}

fn fixed_delay_altered(fda: &mut FixedDelayAltered, buffer: &mut [f32], input: f32) {
  for _ in 0..256 {fda.play(buffer, input, 0.1);}
}

fn fixed_delay_ffi(fda: &mut FixedDelayAltered, buffer: *mut f32, len: usize, input: f32) {
  let buffer = unsafe { std::slice::from_raw_parts_mut(buffer, len) };
  for _ in 0..256 {fda.play(buffer, input, 0.1);}
}

fn fixed_delay_outer(pos: &mut usize, buffer: &mut [f32], input: f32) {
  for _ in 0..256 {delay(buffer, pos, input, 0.1);}
}

fn fixed_delay_outer2(pos: &mut usize, buffer: &mut [f32], input: f32) {
  for _ in 0..256 {outer_delay(buffer, pos, input, 0.1);}
}


pub fn criterion_benchmark_delay(c: &mut Criterion) {
  let mut group = c.benchmark_group("delay");
  let mut d = Delay::new();
  let mut fd = FixedDelay::new();
  let mut fda = FixedDelayAltered::new();

  let mut buffer = [0.0; SIZE];
  let mut ffi_buffer = [0.0; SIZE];

  let mut pos: usize = 0;
  let mut single_buffer = [0.0; SIZE];
  let mut pos2: usize = 0;
  let mut single_buffer2 = [0.0; SIZE];

  let signal = 1.0;
  group.bench_function("interpolated delay", |b| {
    b.iter(|| {
      delay_interpolated(&mut d, signal, &mut buffer);
    })
  });
  
  let signal = 1.0;
  group.bench_function("fixed size delay with bitmask", |b| {
    b.iter(|| {
      fixed_delay(&mut fd, signal);
    })
  });

  let signal = 1.0;
  group.bench_function("fixed size delay - altered", |b| {
    b.iter(|| {
      fixed_delay_altered(&mut fda, &mut buffer, signal);
    })
  });
  
  let signal = 1.0;
  group.bench_function("fixed size delay - ffi", |b| {
    b.iter(|| {
      fixed_delay_ffi(&mut fda, ffi_buffer.as_mut_ptr(), 384000, signal);
    })
  });
  
  let signal = 1.0;
  group.bench_function("fixed size delay - function", |b| {
    b.iter(|| {
      fixed_delay_outer(&mut pos, &mut single_buffer, signal);

    })
  });
  
  let signal = 1.0;
  group.bench_function("fixed size delay - function2", |b| {
    b.iter(|| {
      fixed_delay_outer2(&mut pos2, &mut single_buffer2, signal);

    })
  });
}

pub struct FixedDelayAltered {
  position: usize,
}

pub fn outer_delay(buffer: &mut [f32], pos: &mut usize, input: f32, feedback: f32) -> f32 {
  let len = buffer.len();
  let time = (*pos + len) % len;
  let out = buffer[time];
  *pos %= buffer.len();
  buffer[*pos] = input + (out * feedback);
  *pos += 1;
  out
}
 
impl FixedDelayAltered {
  pub fn play(&mut self, buffer: &mut [f32], input: f32, feedback: f32) -> f32 {
    let len = buffer.len();
    let time = (self.position + len) % len;
    let out = buffer[time];
    self.position %= buffer.len();
    buffer[self.position] = input + (out * feedback);
    self.position += 1;
    out
  }

  pub fn new() -> Self {
    Self {
      position: 0,
    }
  }
}


