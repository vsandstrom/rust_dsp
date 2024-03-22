use core::marker::PhantomData;
use interpolation::interpolation::Interpolation;

pub trait DelayTrait {
  fn new(delay_time: f32, max_delay_time: f32, delay_taps: usize, samplerate: f32) -> Self;
  fn from_samples(buffer_size: usize, delay_taps: usize, samplerate: f32) -> Self;
  fn play(&mut self, sample: f32, feedback: f32) -> f32;
  fn set_taps(&mut self, delay_taps: usize);
  fn set_time(&mut self, delay_time: f32);
}

/// Non interpolating delay, floors the delay in seconds to closest index below
/// in buffer. Reads from current sample and writes to n positions above in the
/// buffer. ( n = delay_taps )
///
/// Single read-, multiple write heads
pub struct Delay {
  buffer: Vec<f32>,
  buffer_size: usize,
  samplerate: f32,
  delay_taps: usize,
  delay_time: f32,
  position: usize,
  pos_mask: usize,
}

fn pow_two(x: usize) -> usize {
  let mut y: usize = 1;
  while x > y { y <<= 1 }
  y
}


impl DelayTrait for Delay {
  /// Create new Delay
  /// max_delay_time >= delay_time * delay_taps,
  /// ex: max_delay_time = 1.0, delay_time = 0.2, delay_taps = 5
  fn new(delay_time: f32, max_delay_time: f32, delay_taps: usize, samplerate: f32) -> Self {
    let buffer_size = pow_two((max_delay_time * samplerate) as usize);
    let buffer = vec![0.0; buffer_size];

    Delay{
      buffer,
      buffer_size,
      delay_time,
      delay_taps,
      samplerate,
      position: 0,
      pos_mask: buffer_size - 1,
    }
  }

  fn from_samples(buffer_size: usize, delay_taps: usize, samplerate: f32) -> Self {
    let buffer_size = pow_two(buffer_size);
    let buffer = vec![0.0; buffer_size];

    Delay{
      buffer,
      buffer_size,
      delay_time: buffer_size as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
      pos_mask: buffer_size - 1,
    }
  }

  fn play(&mut self, sample: f32, feedback: f32) -> f32 {
    let out = self.buffer[self.position];
    self.buffer[self.position] = 0.0;
    for i in 1..=self.delay_taps {
      let mut delay = ((self.delay_time * self.samplerate) as usize * i) + self.position;
      while delay >= self.buffer_size {
        delay -= self.buffer_size;
      }
      self.buffer[delay] += (sample + (out * feedback))  * (0.5/i as f32);
    }
    self.position = (self.position+1) & self.pos_mask;
    out
  }

  fn set_taps(&mut self, delay_taps: usize) {
    self.delay_taps = delay_taps
  }

  fn set_time(&mut self, delay_time: f32) {
    if (delay_time * self.samplerate) as usize >= self.buffer_size {
      self.delay_time = self.buffer_size as f32 / self.samplerate;
      return;
    }
    self.delay_time = delay_time
  }
}


/// Interpolating delay. 
///
/// Single write-, multiple read heads
pub struct IDelay<T> {
  buffer: Vec<f32>,
  buffer_size: usize,
  samplerate: f32,
  delay_taps: usize,
  delay_time: f32,
  position: usize,
  pos_mask: usize,
  _interpolation: PhantomData<T>
}

impl<T> DelayTrait for IDelay<T> where T: Interpolation {
  fn new(delay_time: f32, max_delay_time: f32, delay_taps: usize, samplerate: f32) -> Self {
    let buffer_size = pow_two((max_delay_time * samplerate) as usize);
    let buffer = vec![0.0; buffer_size];

    println!("{}", buffer.len());

    IDelay{
      buffer,
      buffer_size,
      delay_time,
      delay_taps,
      samplerate,
      position: 0,
      pos_mask: buffer_size - 1,
      _interpolation: PhantomData
    }
  }

  fn from_samples(buffer_size: usize, delay_taps: usize, samplerate: f32) -> Self {
    let bsize = pow_two(buffer_size);
    let buffer = vec![0.0; pow_two(buffer_size)];
    println!("{}", buffer.len());

    IDelay{
      buffer,
      buffer_size: bsize,
      delay_time: buffer_size as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
      pos_mask: buffer_size - 1,
      _interpolation: PhantomData
    }
  }

  fn play(&mut self, sample: f32, feedback: f32) -> f32 {
    let mut out = 0.0;

    let del_time = self.delay_time * self.samplerate;
    let read_pos = {
      (1..=self.delay_taps)
        .map(|n| 
          self.position as f32 + (del_time * (n as f32)))
        .collect::<Vec<f32>>()
    };
    // Read from several positions ahead in buffer,
    for (i, pos) in read_pos.iter().enumerate() {
      out += T::interpolate(*pos, &self.buffer, self.buffer_size) / (i+1) as f32  ;
    }

    self.buffer[self.position] = sample + (out * feedback);
    self.position = (self.position+1) & self.pos_mask;
    out
  }

  fn set_taps(&mut self, delay_taps: usize) {
    self.delay_taps = delay_taps;
  }

  fn set_time(&mut self, delay_time: f32) {
    if (delay_time * self.samplerate) as usize >= self.buffer_size {
      self.delay_time = self.buffer_size as f32 / self.samplerate;
      return;
    }
    self.delay_time = delay_time
  }

}


#[cfg(test)]
mod tests {
}
