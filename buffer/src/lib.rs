#![allow(dead_code)]

#[derive(Default, Debug, Clone)]
pub struct Buffer {
  buffer: Vec<f64>,
  samplerate: f64
}

impl Buffer {
  /// Initializes a Buffer of x samples
  pub fn new(len: usize, samplerate: f64) -> Self {
    Buffer{buffer: Vec::with_capacity((len) as usize), samplerate}
  }

  /// Initializes a Buffer from previously populated Vec<f64>
  pub fn new_from_buffer(samplerate: f64, buffer: Vec<f64>) -> Self {
    Buffer{buffer, samplerate}
  }

  /// Initializes a Buffer of n seconds (x = n * samplerate)
  pub fn new_from_seconds(len: f64, samplerate: f64) -> Self {
    Buffer{buffer: Vec::with_capacity((len*samplerate) as usize), samplerate}
  }

  /// Writes and updates buffer at position
  pub fn write(&mut self, sample: f64, position: usize) {
    let mut pos = position;
    while pos >= self.buffer.len() { pos %= self.buffer.len(); }
    self.buffer[pos] = sample;
  }



  pub fn read(&self, position: f64) -> f64{
    self.buffer[position.floor() as usize]
  } 
}

pub trait Interpolation {
  fn read(&self, position: f64) -> f64;
}

// pub trait CubicBuffer { fn read(&self, position: f64) -> f64; } 
// pub trait LinearBuffer { fn read(&self, position: f64) -> f64; }
// pub trait NoneBuffer { fn read(&self, position: f64) -> f64; }

// impl LinearBuffer {
//   fn read(&self, position: f64) -> f64{
//     interpolation::linear(position, &self.buffer)
//   }
// }
//
// impl CubicBuffer {
//   fn read(&self, position: f64) -> f64{
//     interpolation::linear(position, &self.buffer)
//   }
// }
//   
// impl NoneBuffer {
// }

// #[cfg(test)]
// mod tests {
//   use crate::{Buffer, LinearBuffer, CubicBuffer};
//   #[test]
//   fn none_test() {
//     let buf = Buffer::new_from_buffer(48000.0, vec![0.0, 1.0]);
//     let position = 0.5;
//     assert_eq!(0.0, buf.read(position))
//   }
//
//   #[test]
//   fn linear_test() {
//     let buf = Buffer::new_from_buffer(48000.0, vec![0.0, 1.0]);
//     let position = 0.5;
//     assert_eq!(0.5, buf.read(position))
//   }
//
//
//   #[test]
//   fn cubic_test() {
//     let buf = CubicBuffer::new_from_buffer(48000.0, vec![0.0, 1.0, 2.0, 1.0]);
//     let position = 1.5;
//     assert_eq!(1.75, buf.read(position))
//   }
// }
