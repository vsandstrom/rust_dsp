use crate::{dsp::math::is_pow2, interpolation::Interpolation};
use alloc::{vec, vec::Vec};

// pub trait DelayTrait {
//   fn new(length: usize) -> Self;
//   // fn set_time(&mut self, delay_time: f32);
// }

#[derive(Default)]
pub struct Delay {
  position: usize,
}



impl Delay {
  pub fn new() -> Self { Delay { position: 0 } }
  // delay is set in number of samples, but restricted to floats for interpolation 
  // enabling the process.
  pub fn play<T: Interpolation>(&mut self, buffer: &mut [f32], input: f32, delay: f32, feedback: f32) -> f32 {
    let len = buffer.len() as f32;
    let mut time = self.position as f32 + delay;
    while time >= len { time -= len };
    while time < 0.0  { time += len };
    let out = T::interpolate(time, buffer, buffer.len());
    self.position %= buffer.len();
    while self.position > buffer.len() { self.position -= buffer.len() }
    buffer[self.position] = input + (out * feedback);
    self.position += 1;
    out
  }
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



pub struct DelayLine {
  data: Vec<f32>,
  mask: usize,
  read_ptr: usize,
  write_ptr: usize,
}

impl DelayLine {
  pub fn new(offset: usize, size: usize) -> Result<Self, &'static str> {
    if !is_pow2(size) {
      return Err("Size of buffer is not a power of 2");
    }
    if size <= offset {
      return Err("Offset needs to be smaller than N")
    }
    Ok(Self {
      data: vec![0.0; size],
      mask: size-1,
      read_ptr: size - offset,
      write_ptr: 0,
    })
  }

  pub fn read_and_write(&mut self, sample: f32) -> f32 {
    let out = self.data[self.read_ptr];
    self.data[self.write_ptr] = sample;
    self.write_ptr = (self.write_ptr + 1) & self.mask;
    self.read_ptr = (self.read_ptr + 1) & self.mask;
    out
  }

  /// Must be used exactly the same number of times in audio callback as 
  /// `[write]` to keep the delay offet as expected. 
  /// Use `[read_and_write]` if you do not need to do feedback or other
  /// processing that requires you to split the read and write process.
  pub fn read(&mut self) -> f32 {
    let out = self.data[self.read_ptr];
    self.read_ptr = (self.read_ptr + 1) & self.mask;
    out
  }

  /// Must be used exactly the same number of times in audio callback as 
  /// `[read]` to keep the delay offet as expected. 
  /// Use `[read_and_write]` if you do not need to do feedback or other
  /// processing that requires you to split the read and write process.
  pub fn write(&mut self, sample: f32) {
    self.data[self.write_ptr] = sample;
    self.write_ptr = (self.write_ptr + 1) & self.mask;
  }

}
