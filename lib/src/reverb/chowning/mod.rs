use super::Verb;
use super::{Filter, Comb};
use super::Interpolation;


pub struct ChownVerb {
  c1: Comb,
  c2: Comb,
  c3: Comb,
  c4: Comb,
  ccoeffs: [f32; 4],
  a1: Comb,
  a2: Comb,
  a3: Comb,
}

impl Verb for ChownVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new::<901>(samplerate, 0.95, 0.0);
    let mut c2 = Comb::new::<778>(samplerate, 0.95, 0.0);
    let mut c3 = Comb::new::<1011>(samplerate, 0.95, 0.0);
    let mut c4 = Comb::new::<1123>(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);

    let a1 = Comb::new::<125>(samplerate, 0.7, 0.7);
    let a2 = Comb::new::<42>(samplerate, 0.7, 0.7);
    let a3 = Comb::new::<13>(samplerate, 0.7, 0.7);

    Self {
      c1, c2, c3,c4,
      ccoeffs: [ 0.805, 0.827, 0.783, 0.764 ],
      a1, a2, a3
    }
  }

  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out;
    out = self.c1.process(sample) * self.ccoeffs[0];
    out += self.c2.process(out) * self.ccoeffs[1];
    out += self.c3.process(out) * self.ccoeffs[2];
    out += self.c4.process(out) * self.ccoeffs[3];
      
    out = self.a1.process(out);
    out = self.a2.process(out);
    out = self.a3.process(out);
    out
  }
}
