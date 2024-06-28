extern crate interpolation;
extern crate filter;
use filter::{Comb, Filter};
use interpolation::interpolation::Interpolation;

pub struct SchroederVerb {
  c1: Comb<1116>,
  c2: Comb<1188>,
  c3: Comb<1277>,
  c4: Comb<1356>,
  a1: Comb<125>,
  a2: Comb<42>,
  a3: Comb<13>,
}

impl Verb for SchroederVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::<1116>::new(samplerate, 0.95, 0.0);
    let mut c2 = Comb::<1188>::new(samplerate, 0.95, 0.0);
    let mut c3 = Comb::<1277>::new(samplerate, 0.95, 0.0);
    let mut c4 = Comb::<1356>::new(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);

    let a1 = Comb::<125>::new(samplerate, 0.7, 0.7);
    let a2 = Comb::<42>::new(samplerate, 0.7, 0.7);
    let a3 = Comb::<13>::new(samplerate, 0.7, 0.7);

    SchroederVerb{
      c1, c2, c3, c4, 
      a1, a2, a3
    }
  }

  fn process<T:Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out = 0.0;
    out += self.c1.process::<T>(sample);
    out += self.c2.process::<T>(sample);
    out += self.c3.process::<T>(sample);
    out += self.c4.process::<T>(sample);

    out = self.a1.process::<T>(out);
    out = self.a2.process::<T>(out);
    out = self.a3.process::<T>(out);
    out
  }
}

pub trait Verb {
  fn new(samplerate: f32) -> Self;
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32;
}

#[cfg(test)]
mod tests {
}
