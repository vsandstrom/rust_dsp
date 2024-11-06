use crate::interpolation::Interpolation;

pub struct Buffer<const N: usize> {
  pub buffer: Vec<f32>,
  pub size: usize,
  pub samplerate: f32,
  pub position: f32,
}

impl<const N: usize> Buffer<N> {
  pub fn new(samplerate: f32) -> Self {
    let buffer = vec![0.0; N];
    Buffer{
      buffer, 
      size: N,
      position: 0.0,
      samplerate,
    }
  }

  pub fn from_buffer(buffer: [f32;N], samplerate: f32) -> Self {
    Buffer {
      buffer: buffer.to_vec(), 
      size: N,
      position: 0.0,
      samplerate,
    }
  }

  /// Writes and updates buffer at position
  pub fn write(&mut self, sample: f32, position: usize) {
    let mut pos = position;
    while pos >= N { pos -= N; }
    self.buffer[pos] = sample;
  }

  pub fn read<T: Interpolation>(&self, position: f32) -> f32{
    T::interpolate(position, &self.buffer, N)
  } 

  pub fn record(&mut self, sample: f32) -> Option<f32> {
    if self.position as usize >= self.size { return None }
    self.buffer[self.position as usize] = sample;
    self.position += 1.0;
    Some(sample)
  }
}

#[cfg(test)]
mod tests {
  use crate::interpolation::{Floor, Linear, Cubic};
  use super::Buffer;

  #[test]
  fn none_test() {
    let buffer = Buffer::<2>::from_buffer([0.0, 1.0], 48000.0);
    let position = 0.5;
    assert_eq!(0.0, buffer.read::<Floor>(position))
  }

  #[test]
  fn linear_test() {
    let buffer = Buffer::<2>::from_buffer([0.0, 1.0], 48000.0);
    let position = 0.5;
    assert_eq!(0.5, buffer.read::<Linear>(position))
  }

  #[test]
  fn cubic_test() {
    let buffer = Buffer::<4>::from_buffer([0.0, 1.0, 2.0, 1.0], 48000.0);
    let pos = 1.5;
    assert_eq!(1.75, buffer.read::<Cubic>(pos))
  }
  
  #[test]
  fn cubic_test2() {
    let buffer = Buffer::<4>::from_buffer([0.0, 4.0, 2.0, 1.0], 48000.0);
    let pos = 1.5;
    assert_eq!(3.625, buffer.read::<Cubic>(pos))
  }
  
  #[test]
  fn cubic_test3() {
    let buffer = Buffer::<5>::from_buffer([0.0, 4.0, 4.2, 2.0, 1.0], 48000.0);
    let pos = 2.25;
    assert_eq!(3.725, buffer.read::<Cubic>(pos))
  }

  #[test]
  fn linear_wrap_test() {
    let buffer = Buffer::<2>::from_buffer([0.0, 1.0], 48000.0);
    let pos = 2.5;
    assert_eq!(0.5, buffer.read::<Linear>(pos))
  }

  #[test]
  fn cubic_wrap_test() {
    let buffer = Buffer::<5>::from_buffer([0.0, 4.0, 4.2, 2.0, 1.0], 48000.0);
    let pos = 7.25;
    assert_eq!(3.725, buffer.read::<Cubic>(pos))
  }
}
