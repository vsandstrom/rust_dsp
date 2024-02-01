use core::marker::PhantomData;
use buffer::Buffer;
use interpolation::interpolation::{Interpolation, Linear};

pub struct Delay<T> {
  buffer: Buffer<T>,
  samplerate: f32,
  delay_taps: u32,
  delay_time: f32,
  position: usize,
  _interpolation: PhantomData<T>
}

impl<T> Delay<T> 
  where T: Interpolation
{
  pub fn new(delay_time: f32, max_delay_time: f32, delay_taps: u32, samplerate: f32) -> Self {
    // let mut buffer = Buffer::<T>::new((max_delay_time * samplerate) as usize, samplerate);
    let mut buffer = Buffer::<T>::new((max_delay_time * samplerate) as usize, samplerate);
    buffer.init();
    Delay{
      buffer,
      delay_time,
      delay_taps,
      samplerate,
      position: 0,
      _interpolation: PhantomData
    }
  }

  pub fn from_samples(buffer_size: usize, delay_taps: u32, samplerate: f32) -> Self {
    let mut buffer = Buffer::<T>::new(buffer_size, samplerate);
    buffer.init();
    Delay{
      buffer,
      delay_time: buffer_size as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
      _interpolation: PhantomData
    }
  }

  pub fn play(&mut self, sample: f32, feedback: f32) -> f32 {
    let mut out = 0.0;
    // Read delay bounces in buffer
    if self.position >= self.buffer.len() {
      self.position -= self.buffer.len();
    } 

    for i in 0..self.delay_taps {
      // let delay = self.delay_time * self.samplerate * (i as f32 + 1.0);
      let delay = 4.0 * (i as f32 + 1.0);
      let pos = {
        if delay >= self.position as f32 {
          self.buffer.len() as f32 - delay - self.position as f32
        } else {
          self.position as f32 - delay
        }
      };
      out += self.buffer.read(pos);
    }

    // Write the new delay with possible feedback.
    self.buffer.write(sample + (out * feedback), self.position);

    // Wrap read/write-position
    self.position += 1;
    out
  }

  pub fn set_taps(&mut self, delay_taps: u32) {
    self.delay_taps = delay_taps
  }

  pub fn set_time(&mut self, delay_time: f32) {
    self.delay_time = delay_time
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn delay_test() {
    let mut delay = Delay::<Linear>::from_samples(4, 1, 48000.0);
    print!("{} ", delay.play(1.0, 0.0));
    for _ in 0..1000 {
      print!("{} ", delay.play(0.0, 0.0));
    }
    println!();
    assert_eq!(1.0, delay.play(0.0, 0.0))
  }
  
  #[test]
  fn delay_test2() {
    let mut delay = Delay::<Linear>::new(6.0/48000.0, 1.0, 1, 48000.0);
    delay.play(1.0, 0.0);

    delay.play(0.0, 0.0);
    delay.play(0.0, 0.0);
    assert_ne!(1.0, delay.play(0.0, 0.0))
  }
}
