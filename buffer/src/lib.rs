extern crate interpolation;
use interpolation::interpolation::{Interpolation, Floor};
use core::marker::PhantomData;

pub struct Buffer<T> {
  pub buffer: Vec<f32>,
  #[allow(unused)]
  size: usize,
  pub samplerate: f32,
  interpolation: PhantomData<T>
}

impl<T: Interpolation> Buffer<T> {
  pub fn new(size: usize, samplerate: f32) -> Self {
    let bufsize = Buffer::<Floor>::minimum_buf_size(size);
    Buffer{
      buffer: Vec::with_capacity((bufsize) as usize), 
      size,
      samplerate,
      interpolation: PhantomData 
    }
  }


  /// Initializes a Buffer from previously populated Vec<f32>
  pub fn from_buffer(samplerate: f32, buffer: Vec<f32>) -> Self {
    let len = buffer.len();
    Buffer{
      buffer, 
      size: len,
      samplerate,
      interpolation: PhantomData 
    }
  }

  /// Initializes a Buffer of n seconds (x = n * samplerate)
  pub fn from_seconds(size: f32, samplerate: f32) -> Self {
    let bufsize = Buffer::<Floor>::minimum_buf_size((size * samplerate).floor() as usize);
    Buffer{
      buffer: Vec::with_capacity(bufsize),
      size: (size * samplerate) as usize,
      samplerate,
      interpolation: PhantomData
    }
  }
  /// Writes and updates buffer at position
  pub fn write(&mut self, sample: f32, position: usize) {
    let mut pos = position;
    while pos >= self.buffer.len() { pos %= self.buffer.len(); }
    self.buffer[pos] = sample;
  }

  pub fn read(&self, position: f32) -> f32{
    T::interpolate(position, &self.buffer, self.buffer.len())
  } 

  pub fn init(&mut self) {
    for _ in 0..self.size {
      self.buffer.push(0.0);
    }
  }

  fn minimum_buf_size(size: usize) -> usize {
    match size { x if x < 4 => 4, _ => size }
  }

  pub fn len(&self) -> usize {
    self.buffer.len()
  }
}

#[cfg(test)]
mod tests {
  use crate::interpolation::interpolation::{Floor, Linear, Cubic};
  use super::*;

  #[test]
  fn none_test() {
    let buffer = Buffer::<Floor>::from_buffer(48000.0, vec![0.0, 1.0]);
    let position = 0.5;
    assert_eq!(0.0, buffer.read(position))
  }

  #[test]
  fn linear_test() {
    let buffer = Buffer::<Linear>::from_buffer(48000.0, vec![0.0, 1.0]);
    let position = 0.5;
    assert_eq!(0.5, buffer.read(position))
  }

  #[test]
  fn cubic_test() {
    let buffer = Buffer::<Cubic>::from_buffer(48000.0, vec![0.0, 1.0, 2.0, 1.0]);
    let pos = 1.5;
    assert_eq!(1.75, buffer.read(pos))
  }
  
  #[test]
  fn cubic_test2() {
    let buffer = Buffer::<Cubic>::from_buffer(48000.0,  vec![0.0, 4.0, 2.0, 1.0]);
    let pos = 1.5;
    assert_eq!(3.625, buffer.read(pos))
  }
  
  #[test]
  fn cubic_test3() {
    let buffer = Buffer::<Cubic>::from_buffer(48000.0, vec![0.0, 4.0, 4.2, 2.0, 1.0]);
    let pos = 2.25;
    assert_eq!(3.725, buffer.read(pos))
  }

  #[test]
  fn linear_wrap_test() {
    let buffer = Buffer::<Linear>::from_buffer(48000.0, vec![0.0, 1.0]);
    let pos = 2.5;
    assert_eq!(0.5, buffer.read(pos))
  }

  #[test]
  fn cubic_wrap_test() {
    let buffer = Buffer::<Cubic>::from_buffer(48000.0, vec![0.0, 4.0, 4.2, 2.0, 1.0]);
    let pos = 7.25;
    assert_eq!(3.725, buffer.read(pos))
  }
}
