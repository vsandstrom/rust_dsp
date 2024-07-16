extern crate interpolation;
extern crate filter;
use filter::{Comb, Filter};
use interpolation::interpolation::Interpolation;

pub struct SchroederVerb {
  c1: Comb<4799>,
  c2: Comb<4999>,
  c3: Comb<5399>,
  c4: Comb<5801>,
  ccoeffs: [f32; 4],
  a1: Comb<1051>,
  a2: Comb<337>,
  a3: Comb<113>,
}

impl Verb for SchroederVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new(samplerate, 0.95, 0.0);
    let mut c2 = Comb::new(samplerate, 0.95, 0.0);
    let mut c3 = Comb::new(samplerate, 0.95, 0.0);
    let mut c4 = Comb::new(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);


    let a1 = Comb::new(samplerate, 0.7, 0.7);
    let a2 = Comb::new(samplerate, 0.7, 0.7);
    let a3 = Comb::new(samplerate, 0.7, 0.7);

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
    out = self.a1.process::<T>(sample);
    out = self.a2.process::<T>(out);
    out = self.a3.process::<T>(out);

    out += self.c1.process::<T>(out) * self.ccoeffs[0];
    out += self.c2.process::<T>(out) * self.ccoeffs[1];
    out += self.c3.process::<T>(out) * self.ccoeffs[2];
    out += self.c4.process::<T>(out) * self.ccoeffs[3];

    out
  }
}

pub struct ChownVerb {
  c1: Comb<901>,
  c2: Comb<778>,
  c3: Comb<1011>,
  c4: Comb<1123>,
  ccoeffs: [f32; 4],
  a1: Comb<125>,
  a2: Comb<42>,
  a3: Comb<13>,
}

impl Verb for ChownVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new(samplerate, 0.95, 0.0);
    let mut c2 = Comb::new(samplerate, 0.95, 0.0);
    let mut c3 = Comb::new(samplerate, 0.95, 0.0);
    let mut c4 = Comb::new(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);

    let a1 = Comb::new(samplerate, 0.7, 0.7);
    let a2 = Comb::new(samplerate, 0.7, 0.7);
    let a3 = Comb::new(samplerate, 0.7, 0.7);

    Self {
      c1, c2, c3,c4,
      ccoeffs: [ 0.805, 0.827, 0.783, 0.764 ],
      a1, a2, a3
    }
  }

  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out;
    out = self.c1.process::<T>(sample) * self.ccoeffs[0];
    out += self.c2.process::<T>(out) * self.ccoeffs[1];
    out += self.c3.process::<T>(out) * self.ccoeffs[2];
    out += self.c4.process::<T>(out) * self.ccoeffs[3];
      
    out = self.a1.process::<T>(out);
    out = self.a2.process::<T>(out);
    out = self.a3.process::<T>(out);
    out
  }
}


pub struct VikVerb {
  matrix: [f32; 5],
  ccoeffs: [f32; 3],
  diff1: Comb<437>,  // 3.12 m
  diff2: Comb<452>,  // 3.22 m
  diff3: Comb<731>,  // 5.22 m
  diff4: Comb<921>,  // 6.58 m
  diff5: Comb<1109>, // 7.92 m
  c1: Comb<2399>,
  c2: Comb<2809>,
  c3: Comb<3301>,
  a1: Comb<11>,
  a2: Comb<39>,
  a3: Comb<47>,
  bounce: f32,
  early_reflections: f32,

}

impl Verb for VikVerb {
  fn new(samplerate: f32) -> Self {
    let diff1 = Comb::new(samplerate, 0.95, 0.05);
    let diff3 = Comb::new(samplerate, 0.95, 0.05);
    let diff2 = Comb::new(samplerate, 0.95, 0.05);
    let diff4 = Comb::new(samplerate, 0.95, 0.05);
    let diff5 = Comb::new(samplerate, 0.95, 0.05);
    
    let c1 = Comb::new(samplerate, 0.95, 0.0);
    let c2 = Comb::new(samplerate, 0.95, 0.0);
    let c3 = Comb::new(samplerate, 0.95, 0.0);
    
    let a1 = Comb::new(samplerate, 0.7, 0.7);
    let a2 = Comb::new(samplerate, 0.7, 0.7);
    let a3 = Comb::new(samplerate, 0.7, 0.7);
    Self{
      matrix: [0.0; 5],
      diff1, diff2, diff3, diff4, diff5,
      bounce: 0.4,
      early_reflections: 0.2,
      c1, c2, c3, 
      ccoeffs: [0.7301, 0.609, 0.504],
      a1, a2, a3,
    }
  }

  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    self.matrix[0] = self.diff1.process::<T>(sample + self.matrix[3] * self.bounce);
    self.matrix[1] = self.diff2.process::<T>(sample + self.matrix[0] * self.bounce);
    self.matrix[2] = self.diff3.process::<T>(sample + self.matrix[1] * self.bounce);
    self.matrix[3] = self.diff4.process::<T>(sample + self.matrix[4] * self.bounce);
    self.matrix[4] = self.diff5.process::<T>(sample + self.matrix[2] * self.bounce);
    let mut out = self.matrix.iter().sum::<f32>() * 0.2;
    let early = out;
    out += self.c1.process::<T>(out) * self.ccoeffs[0];
    out += self.c2.process::<T>(out) * self.ccoeffs[1];
    out += self.c3.process::<T>(out) * self.ccoeffs[2];

    out = self.a1.process::<T>(out);
    out = self.a2.process::<T>(out);
    out = self.a3.process::<T>(out);
    out + early * self.early_reflections
  }
}

pub trait Verb {
  fn new(samplerate: f32) -> Self;
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32;
}



#[cfg(test)]
mod tests {
}
