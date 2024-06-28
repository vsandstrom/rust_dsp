use interpolation::interpolation::Interpolation;
use dsp::math::is_pow2;


pub trait DelayTrait {
  fn new(delay_taps: usize, samplerate: f32) -> Self;
  // fn play(&mut self, sample: f32, feedback: f32) -> f32;
  fn set_taps(&mut self, delay_taps: usize);
  fn set_time(&mut self, delay_time: f32);
}


/// Non interpolating delay, floors the delay in seconds to closest index below
/// in buffer. Reads from current sample and writes to n positions above in the
/// buffer. ( n = delay_taps )
///
/// Single read-, multiple write heads
pub struct Delay<const N: usize> {
  buffer: Vec<f32>,
  size: usize,
  samplerate: f32,
  delay_time: f32,
  delay_taps: usize,
  position: usize,
  pow2: bool,
  pos_mask: usize,
}

impl<const N: usize> Delay<N> {
  pub fn play(&mut self, sample: f32, feedback: f32) -> f32 {
    let out = self.buffer[self.position];
    self.buffer[self.position] = 0.0;
    for i in 1..=self.delay_taps {
      let mut delay = ((self.delay_time * self.samplerate) as usize * i) + self.position;
      while delay >= self.size {
        delay -= self.size;
      }
      self.buffer[delay] += (sample + (out * feedback))  * (0.5/i as f32);
    }
    match self.pow2 {
      true => {
        self.position = (self.position+1) & self.pos_mask;
      },
      false => {
        self.position = (self.position+1) % self.size;
      }
    }
    out
  }
}

impl<const N: usize> DelayTrait for Delay<N> {
  /// Create new Delay
  /// max_delay_time >= delay_time * delay_taps,
  /// ex: max_delay_time = 1.0, delay_time = 0.2, delay_taps = 5
  fn new(delay_taps: usize, samplerate: f32) -> Self {
    let buffer = vec![0.0; N];

    Delay{
      buffer,
      size: N,
      delay_time: N as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
      pow2: is_pow2(N),
      pos_mask: N - 1,
    }
  }


  fn set_taps(&mut self, delay_taps: usize) {
    self.delay_taps = delay_taps
  }

  fn set_time(&mut self, delay_time: f32) {
    let len = (delay_time * self.samplerate) as usize;
    if len >= N {
      self.pow2 = is_pow2(N);
      self.delay_time = N as f32 / self.samplerate;
    } else {
      self.pow2 = is_pow2(len);
      self.delay_time = delay_time;
    }
  }
}


/// Interpolating delay. 
///
/// Single write-, multiple read heads
pub struct IDelay<const N: usize> {
  buffer: Vec<f32>,
  size: usize,
  samplerate: f32,
  delay_taps: usize,
  delay_time: f32,
  position: usize,
  pow2: bool,
  pos_mask: usize,
}

impl<const N: usize> IDelay<N> {
  pub fn play<T: Interpolation>(&mut self, sample: f32, feedback: f32) -> f32 {
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
      out += T::interpolate(*pos, &self.buffer, self.size) / (i+1) as f32  ;
    }

    self.buffer[self.position] = sample + (out * feedback);
    self.position = (self.position+1) & self.pos_mask;
    out
  }
}

impl<const N: usize> DelayTrait for IDelay<N> {
  fn new(delay_taps: usize, samplerate: f32) -> Self {
    let buffer = vec![0.0; N];

    println!("{}", buffer.len());

    IDelay{
      buffer,
      size: N,
      delay_time: N as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
      pow2: is_pow2(N),
      pos_mask: N - 1,
    }
  }


  fn set_taps(&mut self, delay_taps: usize) {
    self.delay_taps = delay_taps;
  }

  fn set_time(&mut self, delay_time: f32) {
    let len = (delay_time * self.samplerate) as usize;
    if len >= N {
      self.pow2 = is_pow2(N);
      self.delay_time = N as f32 / self.samplerate;
    } else {
      self.pow2 = is_pow2(len);
      self.delay_time = delay_time;
    }
  }
}


#[cfg(test)]
mod tests {
}

