extern crate interpolation;
extern crate filter;

use filter::{Comb, Filter};

use interpolation::interpolation::Floor;

pub struct SchroederVerb {
  cvec: [Comb<Floor>; 4],
  avec: [Comb<Floor>; 3],
}

impl Verb for SchroederVerb {
  fn new(samplerate: f32) -> Self {
    let mut cvec = [
      Comb::<Floor>::new(1116, samplerate, 0.95, 0.0),
      Comb::<Floor>::new(1188, samplerate, 0.95, 0.0),
      Comb::<Floor>::new(1277, samplerate, 0.95, 0.0),
      Comb::<Floor>::new(1356, samplerate, 0.95, 0.0),
    ];

    for i in 0..4 { cvec[i].set_damp(0.3); }

    let avec = [
      Comb::<Floor>::new(125, samplerate, 0.7, 0.7),
      Comb::<Floor>::new(42, samplerate, 0.7, 0.7),
      Comb::<Floor>::new(13, samplerate, 0.7, 0.7),
    ];

    SchroederVerb{cvec, avec}
  }

  fn process(&mut self, sample: f32) -> f32 {
    let mut out = 0.0;
    for i in 0..4 { out += self.cvec[i].process(sample); }
    for i in 0..3 { out = self.avec[i].process(out); }
    out
  }

  fn set_damp(&mut self, damp: f32) {
    for comb in self.cvec.iter_mut() {
      comb.set_damp(damp);
    }
  }
}

pub trait Verb {
  fn new(samplerate: f32) -> Self;
  fn process(&mut self, sample: f32) -> f32;
  fn set_damp(&mut self, damp: f32);
}

#[cfg(test)]
mod tests {
}
