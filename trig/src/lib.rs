use rand::Rng;

pub struct Impulse { samplerate: f32, duration: f32, counter: u32 }
pub struct Dust { samplerate: f32, duration: f32, counter: u32 }

trait Trigger { fn play(&mut self, duration: f32) -> f32; }

impl Impulse {
  pub fn new(duration: f32, samplerate: f32) -> Self {
    Impulse{duration, samplerate, counter: 0}
  }
}

impl Dust {
  pub fn new(duration: f32, samplerate: f32) -> Self {
    Dust{duration, samplerate, counter: 0}
  }
}


impl Trigger for Impulse {
  fn play(&mut self, duration: f32) -> f32 {
    if self.counter >= (self.duration * self.samplerate) as u32 {
      self.duration = duration;
      self.counter = 0;
      return 1.0;
    }
    self.counter += 1;
    0.0
  }
}

impl Trigger for Dust {
  fn play(&mut self, duration: f32) -> f32 {
    if self.counter >= (self.duration * self.samplerate) as u32 {
      let rng = rand::thread_rng().gen_range(0.0..=1.0) * 2.0;
      self.duration = duration * rng;
      self.counter = 0;
      return 1.0;
    }
    self.counter += 1;
    0.0
  }
}


#[cfg(test)]
mod tests {
    use super::*;
}
