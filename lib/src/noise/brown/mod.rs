use super::white::WhiteNoise;
use crate::filter::onepole::Onepole;
use crate::filter::Filter;
use super::TRand;

pub struct BrownNoise {
  rng: TRand,
  lowpass: Onepole
}

impl BrownNoise {
  pub fn new(samplerate: u32, seed: u32) -> Self {
    let mut lowpass = Onepole::new(samplerate);
    // close to DC, could try 1 - 5 Hz
    lowpass.set_cutoff(5.0);
    Self { rng: TRand::new(seed), lowpass }
  }

  pub fn process(&mut self, out: &mut [f32]) {
    out.iter_mut().for_each(|x| {
      let val = self.rng.next() | 0x4000_0000;
      *x = self.lowpass.process(f32::from_bits(val) - 3.0)
    });
  }
}
