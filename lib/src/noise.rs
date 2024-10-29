use rand::{self, Rng};

pub struct Noise {
  counter: u64,
  duration_in_samples: u64,
  current: f32,
  inc: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl Noise {
  pub fn play(&mut self, duration: f32) -> f32 {
    self.counter += 1;
    if self.counter >= self.duration_in_samples {
      self.duration_in_samples = (self.samplerate * duration) as u64;
      self.counter = 0;
      let next = rand::thread_rng().gen_range(-1.0..1.0);
      self.inc = ( next - self.current ) / self.duration_in_samples as f32;
    }
    self.current += self.inc;
    self.current
  }

  pub fn new(samplerate: f32) -> Self {
    Self {
      current: 0.0,
      inc: 0.0,
      counter: 0,
      duration_in_samples: 0,
      samplerate,
      sr_recip: 1.0/ samplerate,
    }
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
      self.sr_recip = 1.0 / samplerate;
      self.samplerate = samplerate;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn poll() {
    let _rnd = Noise::new(48000.0);

    
  }
}
