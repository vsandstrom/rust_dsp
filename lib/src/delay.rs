use crate::interpolation::Interpolation;
use alloc::{vec, vec::Vec};

pub trait DelayTrait {
  fn new(length: usize) -> Self;
  // fn set_time(&mut self, delay_time: f32);
}

pub struct Delay {
  buffer: Vec<f32>,
  position: usize,
}

impl Delay {
  pub fn play<T: Interpolation>(&mut self, input: f32, delay: f32, feedback: f32) -> f32 {
    let len = self.buffer.len() as f32;

    let mut time = self.position as f32 + delay;
    while time >= len { time -= len };
    while time < 0.0  { time += len };
    let out = T::interpolate(time, &self.buffer, self.buffer.len());
    self.position %= self.buffer.len();
    self.buffer[self.position] = input + (out * feedback);
    self.position += 1;
    out
  }
}

impl DelayTrait for Delay {
  /// Delay in samples
  fn new(max_samples: usize) -> Self {
    Delay{
      buffer: vec![0.0; max_samples],
      position: 0,
    }
  }

  // /// Set delay time in samples
  // fn set_time(&mut self, delay: f32) {
  //   self.delay = delay;
  // }
}

/// Constant size delay line.
///
/// Be careful of Stack overflow, works on shorter delay times
/// preferably within reverb or filter chains.
pub struct FixedDelay<const MAXLEN: usize> {
  buffer: [f32; MAXLEN],
  position: usize,
}
 
impl<const MAXLEN:usize> FixedDelay<MAXLEN> {
  pub fn play(&mut self, input: f32, feedback: f32) -> f32 {
    let time = (self.position + MAXLEN) % MAXLEN;
    let out = self.buffer[time];
    self.position %= self.buffer.len();
    self.buffer[self.position] = input + (out * feedback);
    self.position += 1;
    out
  }

  pub fn new() -> Self {
    Self {
      buffer: [0.0; MAXLEN],
      position: 0,
    }
  }
}

impl<const MAXLEN: usize> Default for FixedDelay<MAXLEN> {
  fn default() -> Self {
    Self {
      buffer: [0.0; MAXLEN],
      position: 0,
    }
  }
}

/// A non interpolated delay function, where state management of buffer and position is handled
/// elsewhere.
#[inline]
pub fn delay(buffer: &mut [f32], pos: &mut usize, input: f32, feedback: f32) -> f32 {
  let len = buffer.len();
  let time = (*pos + len) % len;
  let out = buffer[time];
  *pos %= buffer.len();
  buffer[*pos] = input + (out * feedback);
  *pos += 1;
  out
}

