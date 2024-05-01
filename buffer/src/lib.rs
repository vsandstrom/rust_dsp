extern crate interpolation;
use interpolation::interpolation::{InterpolationConst, Floor};
use core::marker::PhantomData;

pub struct Buffer<T, const N: usize> {
  pub buffer: [f32; N],
  #[allow(unused)]
  pub size: usize,
  pub samplerate: f32,
  pub position: f32,
  interpolation: PhantomData<T>
}

impl<T: InterpolationConst, const N: usize> Buffer<T, N> {
  pub fn new(samplerate: f32) -> Self {
    let buffer = [0.0; N];
    Buffer{
      buffer, 
      size: N,
      position: 0.0,
      samplerate,
      interpolation: PhantomData 
    }
  }

  pub fn from_buffer(buffer: [f32;N], samplerate: f32) -> Self {
    Buffer {
      buffer, 
      size: N,
      position: 0.0,
      samplerate,
      interpolation: PhantomData 
    }
  }

  /// Writes and updates buffer at position
  pub fn write(&mut self, sample: f32, position: usize) {
    let mut pos = position;
    while pos >= self.buffer.len() { pos %= N; }
    self.buffer[pos] = sample;
  }

  pub fn read(&self, position: f32) -> f32{
    T::interpolate(position, &self.buffer, N)
  } 

  pub fn record(&mut self, sample: f32) {
    self.buffer[self.position as usize] = sample;
    self.position += 1.0;
  }
}

#[cfg(test)]
mod tests {
  use crate::interpolation::interpolation::{Floor, Linear, Cubic};
  use super::*;

  #[test]
  fn none_test() {
    let buffer = Buffer::<Floor, 2>::from_buffer([0.0, 1.0], 48000.0);
    let position = 0.5;
    assert_eq!(0.0, buffer.read(position))
  }

  #[test]
  fn linear_test() {
    let buffer = Buffer::<Linear, 2>::from_buffer([0.0, 1.0], 48000.0);
    let position = 0.5;
    assert_eq!(0.5, buffer.read(position))
  }

  #[test]
  fn cubic_test() {
    let buffer = Buffer::<Cubic, 4>::from_buffer([0.0, 1.0, 2.0, 1.0], 48000.0);
    let pos = 1.5;
    assert_eq!(1.75, buffer.read(pos))
  }
  
  #[test]
  fn cubic_test2() {
    let buffer = Buffer::<Cubic, 4>::from_buffer([0.0, 4.0, 2.0, 1.0], 48000.0);
    let pos = 1.5;
    assert_eq!(3.625, buffer.read(pos))
  }
  
  #[test]
  fn cubic_test3() {
    let buffer = Buffer::<Cubic, 5>::from_buffer([0.0, 4.0, 4.2, 2.0, 1.0], 48000.0);
    let pos = 2.25;
    assert_eq!(3.725, buffer.read(pos))
  }

  #[test]
  fn linear_wrap_test() {
    let buffer = Buffer::<Linear, 2>::from_buffer([0.0, 1.0], 48000.0);
    let pos = 2.5;
    assert_eq!(0.5, buffer.read(pos))
  }

  #[test]
  fn cubic_wrap_test() {
    let buffer = Buffer::<Cubic, 5>::from_buffer([0.0, 4.0, 4.2, 2.0, 1.0], 48000.0);
    let pos = 7.25;
    assert_eq!(3.725, buffer.read(pos))
  }
}
