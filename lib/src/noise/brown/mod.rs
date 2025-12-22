use super::white::WhiteNoise;
use crate::filter::onepole::Onepole;
use crate::filter::Filter;

pub struct BrownNoise {
  noise: WhiteNoise,
  lowpass: Onepole
}

impl BrownNoise {
  pub fn new(samplerate: u32) -> Self {
    let mut lowpass = Onepole::new(samplerate);
    // close to DC, could try 1 - 5 Hz
    lowpass.set_cutoff(5.0);
    Self { noise: WhiteNoise {}, lowpass }
  }

  pub fn play(&mut self) -> f32 {
    self.lowpass.process(WhiteNoise::play())
  }
}
