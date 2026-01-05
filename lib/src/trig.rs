use crate::noise::Prng;


pub struct Impulse { 
  samplerate: f32,
  duration: f32,
  counter: u32,
}

pub struct Dust {
  samplerate: f32,
  duration: f32,
  counter: u32,
  rng: Prng
}

pub struct Trigger {
  samplerate: f32,
  duration: f32,
  counter: u32,
  random: bool,
  rng: Prng
}


pub trait TrigTrait { 
  fn play(&mut self, duration: f32) -> f32; 
  fn bind(&mut self, duration: f32, func: &mut impl FnMut());
  fn set_samplerate(&mut self, samplerate: f32);
}

impl Impulse {
  pub fn new(samplerate: u32) -> Self {
    Self { samplerate: samplerate as f32, duration: 0.0, counter: 0 }
  }
}

impl TrigTrait for Impulse {
  fn play(&mut self, duration: f32) -> f32 {
    if self.counter >= (self.duration * self.samplerate) as u32 {
      self.duration = duration;
      self.counter = 0;
      return 1.0;
    }
    self.counter += 1;
    0.0
  }

  fn bind(&mut self, duration: f32, func: &mut impl FnMut()) {
    if self.counter >= (self.duration * self.samplerate) as u32 {
      self.duration = duration;
      self.counter = 0;
      func();
    }
    self.counter += 1;
  }

  fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
  }
}

impl Dust {
  pub fn new(samplerate: u32, seed: u32) -> Self {
    Self { samplerate: samplerate as f32, duration: 0.0, counter: 0, rng: Prng::new(seed) }
  }
}

impl TrigTrait for Dust {
  fn play(&mut self, duration: f32) -> f32 {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
      return 0.0;
    }
    let rng = self.rng.frand_unipolar() * 2.0;
    self.duration = duration * rng;
    self.counter = 0;
    1.0
  }

  fn bind(&mut self, duration: f32, func: &mut impl FnMut()) {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
    }
    let rng = self.rng.frand_unipolar() * 2.0;
    self.duration = duration * rng;
    self.counter = 0;
    func();
  }

  fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
  }
}

impl Trigger {
  pub fn new(samplerate: u32, seed: u32) -> Self {
    Self { samplerate: samplerate as f32, duration: 0.0, counter: 0, random: false, rng: Prng::new(seed) }
  }
}
impl TrigTrait for Trigger {
  fn play(&mut self, duration: f32) -> f32 {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
      return 0.0;
    }
    if self.random {
      let rng = self.rng.frand_unipolar() * 2.0;
      self.duration = duration * rng;
      self.counter = 0;
    } else {
      self.duration = duration;
      self.counter = 0;
    }
    1.0
      
  }

  fn bind(&mut self, duration: f32, func: &mut impl FnMut()) {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
      return;
    }
    if self.random {
      let rng = self.rng.frand_unipolar() * 2.0;
      self.duration = duration * rng;
      self.counter = 0;
    } else {
      self.duration = duration;
      self.counter = 0;
    }
    func()
  }

  fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
  }

}

impl Trigger {
  pub fn set_random(&mut self, random: bool) {
    self.random = random;
  }
}




// #[cfg(test)]
// mod tests {
//     use super::*;
// }
