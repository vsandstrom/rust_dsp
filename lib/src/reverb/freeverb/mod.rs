use crate::filter::{Filter, comb::{Comb, LPComb}};
use super::Verb;
use std::arch::asm;


pub struct Freeverb {
  lpc: [LPComb; 8],
  ap: [Comb; 4],
}

// impl Freeverb {
//   /// values of `[damp]` should be 0.0 < damp < 1.0
//   /// to have a stable behavior.
//   /// values below 0.0 will explode the reverb.
//   pub fn set_damp(&mut self, damp: f32) {
//     self.lpc.iter_mut().for_each(|x| x.set_damp(damp));
//   }
// }

impl Freeverb {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for Freeverb {
  fn default() -> Self {
    let mut lpc =  [
      LPComb::new::<1557>(0.0, 0.84),
      LPComb::new::<1617>(0.0, 0.84),
      LPComb::new::<1491>(0.0, 0.84),
      LPComb::new::<1422>(0.0, 0.84),
      LPComb::new::<1277>(0.0, 0.84),
      LPComb::new::<1356>(0.0, 0.84),
      LPComb::new::<1188>(0.0, 0.84),
      LPComb::new::<1116>(0.0, 0.84),
    ];

    lpc.iter_mut().for_each(|l| l.set_damp(0.0));
    Self {
      lpc,
      ap: [
        Comb::new::<225>(0.5, 0.5),
        Comb::new::<556>(0.5, 0.5),
        Comb::new::<441>(0.5, 0.5),
        Comb::new::<241>(0.5, 0.5)
      ]
    }
  }

}

impl Verb for Freeverb {


  #[cfg(target_arch="x86")]
  fn process<T: crate::interpolation::Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out = self.lpc
      .iter_mut()
      .fold(0.0, |acc, lpcomb| 
        acc + lpcomb.process(sample));

    out = self.ap[0].process(out);
    out = self.ap[1].process(out);
    out = self.ap[2].process(out);
    self.ap[3].process(out)
  }

  #[cfg(not(target_arch="x86"))]
  fn process<T: crate::interpolation::Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out = self.lpc
      .iter_mut()
      .fold(0.0, |acc, lpcomb| 
        acc + lpcomb.process(sample));

    out = self.ap[0].process(out);
    out = self.ap[1].process(out);
    out = self.ap[2].process(out);
    self.ap[3].process(out)
      
  }


  fn set_damp(&mut self, damp: f32) {
    let damp = if damp < 0.0 { 0.0 } else { damp };
    self.lpc.iter_mut().for_each(|op| op.set_damp(damp)); 
  }
}
