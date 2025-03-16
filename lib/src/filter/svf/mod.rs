use super::Filter;
use super::MultiModeTrait;


pub struct SVFilter {
  ic1eq: f32,
  ic2eq: f32,
  a1: f32, 
  a2: f32, 
  a3: f32,
  m0: f32,
  m1: f32,
  m2: f32,
}

impl Default for SVFilter {
  fn default() -> Self {
    Self{
      ic1eq: 0.0,
      ic2eq: 0.0,
      a1: 0.0, a2: 0.0, a3: 0.0,
      m0: 0.0, m1: 0.0, m2: 0.0
    }
      
  }
}

impl SVFilter {
  pub fn new() -> Self {
    Self::default()
  }
}

impl MultiModeTrait for SVFilter {
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
    let v1 = self.a1 * self.ic1eq + self.a2 * v3;
    let v2 = self.ic2eq + self.a2 * self.ic1eq + self.a3 * v3;
    self.ic1eq = 2.0 * v1 - self.ic1eq;
    self.ic2eq = 2.0 * v2 - self.ic2eq;
    self.m0 * sample + self.m1 * v1 + self.m2 * v2
  }

  fn calc_lpf(&mut self, w: f32, q: f32) {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    self.a1 = 1.0 / (1.0 + g * (g + k));
    self.a2 = g * self.a1;
    self.a3 = g * self.a2;
    self.m0 = 0.0;
    self.m1 = 0.0;
    self.m2 = 1.0;
  }
  
  fn calc_bpf(&mut self, w: f32, q: f32) {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    self.a1 = 1.0 / (1.0 + g * (g + k));
    self.a2 = g * self.a1;
    self.a3 = g * self.a2;
    self.m0 = 0.0;
    self.m1 = 1.0;
    self.m2 = 0.0;
      
  }

  fn calc_hpf(&mut self, w: f32, q: f32) {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    self.a1 = 1.0 / (1.0 + g * (g + k));
    self.a2 = g * self.a1;
    self.a3 = g * self.a2;
    self.m0 = 1.0;
    self.m1 = -k;
    self.m2 = -1.0;
      
  }

  fn calc_peq(&mut self, w: f32, q: f32, gain: f32) {
    let a = f32::powf(10.0, gain / 40.0);
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    self.a1 = 1.0 / (1.0 + g * (g + k));
    self.a2 = g * self.a1;
    self.a3 = g * self.a2;
    self.m0 = 1.0;
    self.m1 = k * (a * a - 1.0);
    self.m2 = 0.0;
      
  }

  fn calc_notch(&mut self, w: f32, q: f32) {
    let g = f32::tan(w/2.0);
    let k = 1.0/q;
    self.a1 = 1.0 / (1.0 + g * (g + k));
    self.a2 = g * self.a1;
    self.a3 = g * self.a2;
    self.m0 = 1.0;
    self.m1 = -k;
    self.m2 = 0.0;
      
  }
}
