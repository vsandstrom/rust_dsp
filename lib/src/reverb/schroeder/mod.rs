use super::Verb;
use super::{Filter, Comb};
use super::Interpolation;

pub struct SchroederVerb {
  c1: Comb,
  c2: Comb,
  c3: Comb,
  c4: Comb,
  ccoeffs: [f32; 4],
  a1: Comb,
  a2: Comb,
  a3: Comb,
}

impl Verb for SchroederVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new::<4799>(0.95, 0.0);
    let mut c2 = Comb::new::<4999>(0.95, 0.0);
    let mut c3 = Comb::new::<5399>(0.95, 0.0);
    let mut c4 = Comb::new::<5801>(0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);


    let a1 = Comb::new::<1051>(0.7, 0.7);
    let a2 = Comb::new::<337>(0.7, 0.7);
    let a3 = Comb::new::<113>(0.7, 0.7);

    SchroederVerb{
      c1, c2, c3, c4, 
      ccoeffs: [
        0.742, 
        0.733, 
        0.715, 
        0.697
      ],
      a1, a2, a3 
    }            
  }              

  fn process<T:Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out;
    out = self.a1.process(sample);
    out = self.a2.process(out);
    out = self.a3.process(out);

    out += self.c1.process(out) * self.ccoeffs[0];
    out += self.c2.process(out) * self.ccoeffs[1];
    out += self.c3.process(out) * self.ccoeffs[2];
    out += self.c4.process(out) * self.ccoeffs[3];

    out
  }
}
