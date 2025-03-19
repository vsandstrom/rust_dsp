use super::Filter;

pub trait SVFTrait {
  fn update(&mut self, coeffs: SVFCoeffs);
}

#[derive(Default)]
pub struct SVFCoeffs { a1: f32, a2: f32, a3: f32, m0: f32, m1: f32, m2: f32, k: f32 }

pub struct SVFilter {
  ic1eq: f32,
  ic2eq: f32,
  c: SVFCoeffs,
}

impl Default for SVFilter {
  fn default() -> Self {
    Self{
      ic1eq: 0.0,
      ic2eq: 0.0,
      c: SVFCoeffs::default(),
    }
  }
}

impl SVFilter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_mode(&mut self, mode: f32) {
    todo!("moving between filter modes is not yet implemented")
  }
}

impl Filter for SVFilter {
  fn process(&mut self, sample: f32) -> f32 {
    // v0 is sample
    //
    // v3 = v0 - ic2eq
    // v1 = a1 * ic1eq + a2 * v3
    // v2 = ic2eq + a2 * ic1eq + a3 * v3
    // ic1eq = 2 * v1 - ic1eq
    // ic2eq = 2 * v2 - ic2eq
    //
    // output = m0 * v0 + m1 * v1 + m2 * v2

    let v3 = sample - self.ic2eq;
    let v1 = self.c.a1 * self.ic1eq + self.c.a2 * v3;
    let v2 = self.ic2eq + self.c.a2 * self.ic1eq + self.c.a3 * v3;
    self.ic1eq = 2.0 * v1 - self.ic1eq;
    self.ic2eq = 2.0 * v2 - self.ic2eq;
    self.c.m0 * sample + self.c.m1 * v1 + self.c.m2 * v2
  }
}

impl SVFTrait for SVFilter {
  fn update(&mut self, coeffs: SVFCoeffs) {
      self.c = coeffs;
  }
}

impl SVFCoeffs {
  #[inline]
  pub fn lpf(w: f32, q: f32) -> Self {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 0.0;
    let m1 = 0.0;
    let m2 = 1.0;
    Self{k, a1, a2, a3, m0, m1, m2}
  }
  
  #[inline]
  pub fn bpf(w: f32, q: f32) -> Self {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 0.0;
    let m1 = 1.0;
    let m2 = 0.0;
    Self{k, a1, a2, a3, m0, m1, m2}
      
  }

  #[inline]
  pub fn hpf(w: f32, q: f32) -> Self {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 1.0;
    let m1 = -k;
    let m2 = -1.0;
    Self{k, a1, a2, a3, m0, m1, m2}
  }

  #[inline]
  pub fn peq(w: f32, q: f32, gain: f32) -> Self {
    let a = f32::powf(10.0, gain / 40.0);
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 1.0;
    let m1 = k * (a * a - 1.0);
    let m2 = 0.0;
    Self{k, a1, a2, a3, m0, m1, m2}
  }

  #[inline]
  pub fn notch(w: f32, q: f32) -> Self {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 1.0;
    let m1 = -k;
    let m2 = 0.0;
    Self{k, a1, a2, a3, m0, m1, m2}
  }

  #[inline]
  pub fn low_shelf(w: f32, q: f32, gain: f32) -> Self { 
    let a = f32::powf(10.0, gain / 40.0);
    let g = f32::tan(w/2.0) / f32::sqrt(gain);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = 1.0;
    let m1 = k * (a - 1.0);
    let m2 = a * a - 1.0;
    Self{k, a1, a2, a3, m0, m1, m2}
  }

  #[inline]
  pub fn high_shelf(&mut self, w: f32, q: f32, gain: f32) -> Self { 
    let a = f32::powf(10.0, gain / 40.0);
    let g = f32::tan(w/2.0) * f32::sqrt(gain);
    let k = 1.0/q;
    let a1 = 1.0 / (1.0 + g * (g + k));
    let a2 = g * a1;
    let a3 = g * a2;
    let m0 = a * a;
    let m1 = k * (1.0 - a) * a;
    let m2 = 1.0 - a * a;
    Self{k, a1, a2, a3, m0, m1, m2}
  }
}
