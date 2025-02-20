use crate::filter::onepole::Onepole;
use super::Verb;
use super::{Filter, Comb};
use super::Interpolation;

pub struct SchroederVerb {
  c: [Comb; 4],
  l: [Onepole; 4],
  ccoeffs: [f32; 4],
  a: [Comb; 3],
}

impl Verb for SchroederVerb {
  fn new() -> Self {
    let mut l = std::array::from_fn(|_| Onepole::new());
    l.iter_mut().for_each(|l| l.set_coeff(0.8));

    Self {
      c: [
        Comb::new::<4799>(0.95, 0.0),
        Comb::new::<4999>(0.95, 0.0),
        Comb::new::<5399>(0.95, 0.0),
        Comb::new::<5801>(0.95, 0.0),
      ],
      l,
      ccoeffs: [ 0.742, 0.733, 0.715, 0.697 ],
      a: [
        Comb::new::<1051>(0.7, 0.7),
        Comb::new::<337>(0.7, 0.7),
        Comb::new::<113>(0.7, 0.7)
      ]
    }            
  }              

  fn process<T:Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out = sample;
    out = self.a[0].process(out);
    out = self.a[1].process(out);
    out = self.a[2].process(out);

    [
      self.l[0].process(self.c[0].process(out) * self.ccoeffs[0]),
      self.l[1].process(self.c[1].process(out) * self.ccoeffs[1]),
      self.l[2].process(self.c[2].process(out) * self.ccoeffs[2]),
      self.l[3].process(self.c[3].process(out) * self.ccoeffs[3]),
    ].iter().sum()
  }
}
