use crate::noise::white;
use crate::filter::onepole::Onepole;
use crate::filter::Filter;
use super::Prng;

pub struct Noise {
  rng: Prng,
  lowpass: Onepole
}

impl Noise {
  pub fn new(seed: u32, samplerate: u32) -> Self {
    let mut lowpass = Onepole::new(samplerate);
    // close to DC, could try 1 - 5 Hz
    lowpass.set_cutoff(2.0);
    Self { 
      rng: Prng::new(seed),
      lowpass 
    }
  }

  pub fn play_block(&mut self, out: &mut [f32]) {
    out.iter_mut().for_each(|x| {
      *x = self.lowpass.process(self.rng.frand_bipolar())
    });
  }

  pub fn play(&mut self) -> f32 {
    self.lowpass.process(self.rng.frand_bipolar())
  }
  
  pub fn play_control(&mut self) -> f32 {
    self.lowpass.process(self.rng.frand_unipolar())
  }
}
