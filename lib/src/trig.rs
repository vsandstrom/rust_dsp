use rand::Rng;

pub struct Impulse { 
  samplerate: f32,
  duration: f32,
  counter: u32 
}

pub struct Dust {
  samplerate: f32,
  duration: f32,
  counter: u32
}

pub struct Trigger {
  samplerate: f32,
  duration: f32,
  counter: u32,
  random: bool
}


pub trait TrigTrait { 
  fn new(samplerate: f32) -> Self;
  fn play(&mut self, duration: f32) -> f32; 
  fn bind(&mut self, duration: f32, func: &mut impl FnMut());
  fn set_samplerate(&mut self, samplerate: f32);
}

impl TrigTrait for Impulse {
  fn new(samplerate: f32) -> Self {
    Self{duration: 0.0, samplerate, counter: 0}
  }

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

impl TrigTrait for Dust {
  fn new(samplerate: f32) -> Self {
    Self{duration: 0.0, samplerate, counter: 0}
  }

  fn play(&mut self, duration: f32) -> f32 {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
      return 0.0;
    }
    let mut rng = rand::thread_rng();
    let rng = rng.gen_range(0.0..=2.0);
    self.duration = duration * rng;
    self.counter = 0;
    1.0
  }

  fn bind(&mut self, duration: f32, func: &mut impl FnMut()) {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
    }
    let mut rng = rand::thread_rng();
    let rng = rng.gen_range(0.0..=2.0);
    self.duration = duration * rng;
    self.counter = 0;
    func();
  }

  fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
  }
}

impl TrigTrait for Trigger {
  fn new(samplerate: f32) -> Self {
    Self{duration: 0.0, samplerate, counter: 0, random: false}
  }

  fn play(&mut self, duration: f32) -> f32 {
    if self.counter < (self.duration * self.samplerate) as u32 {
      self.counter += 1;
      return 0.0;
    }
    if self.random {
      let mut rng = rand::thread_rng();
      let rng = rng.gen_range(0.0..=2.0);
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
      let mut rng = rand::thread_rng();
      let rng = rng.gen_range(0.0..=2.0);
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
