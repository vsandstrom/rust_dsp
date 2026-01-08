use super::Prng;

pub struct ExpensiveNoise {
  counter: u64,
  duration_in_samples: u64,
  current: f32,
  inc: f32,
  samplerate: u32,
  sr_recip: f32,
  rng: Prng
}

impl ExpensiveNoise {
  /// duration is in seconds. 
  /// if you want to play audiorate, use the reciprocal of the frequency you want
  /// ```
  /// use rust_dsp::noise::expensive::ExpensiveNoise;
  ///
  /// let mut noise = ExpensiveNoise::new(48000);
  /// let out = noise.play(1.0/440.0); // ≈ 0.00227272727
  /// ```
  pub fn play(&mut self, duration: f32) -> f32 {
    self.counter += 1;
    if self.counter >= self.duration_in_samples {
      self.duration_in_samples = (self.samplerate as f32 * duration) as u64;
      self.counter = 0;
      let next = self.rng.frand_bipolar();
      self.inc = ( next - self.current ) / self.duration_in_samples as f32;
    }
    self.current += self.inc;
    self.current
  }

  pub fn new(samplerate: u32, seed: u32) -> Self {
    Self {
      current: 0.0,
      inc: 0.0,
      counter: 0,
      duration_in_samples: 0,
      samplerate,
      sr_recip: 1.0/ samplerate as f32,
      rng: Prng::new(seed),
    }
  }

  pub fn set_samplerate(&mut self, samplerate: u32) {
      self.sr_recip = 1.0 / samplerate as f32;
      self.samplerate = samplerate;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn poll() {
    let _rnd = ExpensiveNoise::new(48000, 1234);
  }
}
