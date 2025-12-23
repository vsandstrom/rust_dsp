use crate::noise::white;
use crate::filter::onepole::Onepole;
use crate::filter::Filter;
use super::TRand;

pub struct BrownNoise {
  noise: white::Noise,
  lowpass: Onepole
}

impl BrownNoise {
  pub fn new(samplerate: u32) -> Self {
    let mut lowpass = Onepole::new(samplerate);
    // close to DC, could try 1 - 5 Hz
    lowpass.set_cutoff(2.0);
    Self { 
      noise: white::Noise::default(), 
      lowpass 
    }
  }

  pub fn process_block(&mut self, out: &mut [f32]) {
    out.iter_mut().for_each(|x| {
      *x = self.lowpass.process(self.noise.process())
    });
  }

  pub fn process(&mut self) -> f32 {
    self.lowpass.process(self.noise.process())
  }
}
