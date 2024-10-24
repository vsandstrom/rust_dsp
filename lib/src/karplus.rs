

pub struct Karplus {
  samplerate: f32,
  sr_recip: f32
}

impl Karplus {
  pub fn new(samplerate: f32) -> Self {
    Self {
      samplerate,
      sr_recip: 1.0 / samplerate
    }

  }


  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }
}
