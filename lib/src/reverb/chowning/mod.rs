use crate::filter::Onepole;

use super::Verb;
use super::{Filter, Comb};
use super::Interpolation;

pub struct ChownVerb {
  c: [Comb; 4],
  l: [Onepole; 4],
  ccoeffs: [f32; 4],
  a: [Comb; 3]
}

impl Verb for ChownVerb {
  fn new() -> Self {
    let mut l = std::array::from_fn(|_| Onepole::new());
    l.iter_mut().for_each(|l| l.set_damp(0.3));
    Self {
      l,
      c: [
        Comb::new::<901>(0.95, 0.0),
        Comb::new::<778>(0.95, 0.0),
        Comb::new::<1011>(0.95, 0.0),
        Comb::new::<1123>(0.95, 0.0),
      ],
      ccoeffs: [ 0.805, 0.827, 0.783, 0.764 ],
      a: [
        Comb::new::<125>(0.7, 0.7),
        Comb::new::<42>(0.7, 0.7),
        Comb::new::<13>(0.7, 0.7),
      ]
    }
  }

  #[inline(never)]
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out = 0.0;
    out += self.l[0].process(self.c[0].process(sample) * self.ccoeffs[0]);
    out += self.l[1].process(self.c[1].process(sample) * self.ccoeffs[1]);
    out += self.l[2].process(self.c[2].process(sample) * self.ccoeffs[2]);
    out += self.l[3].process(self.c[3].process(sample) * self.ccoeffs[3]);
    self.a[0].process(out);
    self.a[1].process(out);
    self.a[1].process(out)
  }
}
