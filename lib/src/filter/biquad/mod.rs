pub mod filterbank;

pub struct BiquadCoeffs { 
  a1: f32,  
  a2: f32,
  b0: f32,
  b1: f32,
  b2: f32
}
  
pub trait BiquadTrait {
  fn process(&mut self, sample: f32) -> f32;
  fn set_coeffs(&mut self, coeffs: BiquadCoeffs);
}



#[inline]
pub fn calc_lpf(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = (-1.0 * w.cos()) / a0 ;
    let a2 = (1.0 - alpha) / a0;

    let b1 = (1.0 - w.cos()) / a0;
    let b0 = b1 / 2.0 / a0;
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
}
  
#[inline]
pub fn calc_bpf(w: f32, q: f32) -> BiquadCoeffs {
  let alpha = w.sin() / (2.0 * q);
  
  let a0 = 1.0 + alpha;
  let a1 = (-1.0 * w.cos()) / a0;
  let a2 = (1.0 - alpha) / a0;

  let b0 = alpha / a0;
  let b1 = 0.0;
  let b2 = -alpha / a0;
  BiquadCoeffs{a1, a2, b0, b1, b2}
}

#[inline]
pub fn calc_hpf(w: f32, q: f32) -> BiquadCoeffs {
  let alpha = w.sin() / (2.0 * q);
  let a0 = 1.0 + alpha;
  let a1 = -1.0 * w.cos() / a0;
  let a2 = 1.0 - alpha / a0;

  let b0 = (1.0 + w.cos()) / 2.0 / a0;
  let b1 = -(b0 * 2.0);
  let b2 = b0;
  BiquadCoeffs{a1, a2, b0, b1, b2}
}

#[inline]
pub fn calc_notch(w: f32, q: f32) -> BiquadCoeffs {
  let alpha = w.sin() / (2.0 * q);
  let a0 = 1.0 + alpha;
  let a1 = -2.0 * w.cos() / a0;
  let a2 = (1.0 - alpha) / a0;

  let b0 = 1.0 / a0;
  let b1 = a1;
  let b2 = b0;
  BiquadCoeffs{a1, a2, b0, b1, b2}
}

#[inline]
pub fn calc_peq(w: f32, q: f32) -> BiquadCoeffs {
  todo!()
}

#[derive(Clone, Copy)]
pub struct Biquad {
  x1: f32,
  x2: f32,
  y1: f32,
  y2: f32,
  pub(super) a1: f32,
  pub(super) a2: f32,
  pub(super) b0: f32,
  pub(super) b1: f32,
  pub(super) b2: f32,
  prev_in: f32,
  prev_out: f32
}

#[derive(Clone, Copy)]
pub struct Biquad4 {
  x1_1: f32,
  x1_2: f32,
  y1_1: f32,
  y1_2: f32,
  x2_1: f32,
  x2_2: f32,
  y2_1: f32,
  y2_2: f32,
  a1: f32,
  a2: f32,
  b0: f32,
  b1: f32,
  b2: f32,
  prev_in: f32,
  prev_out: f32
}

#[derive(Clone, Copy)]
pub struct Biquad8 {
  x1_1: f32,
  x1_2: f32,
  y1_1: f32,
  y1_2: f32,
  x2_1: f32,
  x2_2: f32,
  y2_1: f32,
  y2_2: f32,
  x3_1: f32,
  x3_2: f32,
  y3_1: f32,
  y3_2: f32,
  x4_1: f32,
  x4_2: f32,
  y4_1: f32,
  y4_2: f32,
  a1: f32,
  a2: f32,
  b0: f32,
  b1: f32,
  b2: f32,
  prev_in: f32,
  prev_out: f32
}

impl Default for Biquad {
  fn default() -> Self { Self::new() }
}

impl Biquad {

  pub fn new() -> Self {
    Self {
      x1: 0.0, x2: 0.0,
      y1: 0.0, y2: 0.0,
      a1: 0.0, a2: 0.0,
      b0: 0.0, b1: 0.0, b2: 0.0,
      prev_in: 0.0,
      prev_out: 0.0
    }
  }
  
  fn calc_next(&self, input: f32) -> f32 {
      self.b0 * input 
      + self.b1 * self.x1 
      + self.b2 * self.x1
      - self.a1 * self.y1
      - self.a2 * self.y1
  }
}

impl BiquadTrait for Biquad {
  // Direct form I
  fn process(&mut self, sample: f32) -> f32 {
    let output = 
      self.b0 * sample 
      + self.b1 * self.x1 
      + self.b2 * self.x2
      - self.a1 * self.y1
      - self.a2 * self.y2;

    self.x2 = self.x1;
    self.x1 = sample;
    self.y2 = self.y1;
    self.y1 = output;
    output
  }

  fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.a1 = coeffs.a1;
    self.a2 = coeffs.a2;
    self.b0 = coeffs.b0;
    self.b1 = coeffs.b1;
    self.b2 = coeffs.b2;
  }

}

impl BiquadTrait for Biquad4 {
  fn process(&mut self, sample: f32) -> f32 {
    let mut output = 
      self.b0 * sample 
      + self.b1 * self.x1_1 
      + self.b2 * self.x1_2
      - self.a1 * self.y1_1
      - self.a2 * self.y1_2;

    self.x1_2 = self.x1_1;
    self.x1_1 = sample;
    self.y1_2 = self.y1_1;
    self.y1_1 = output;
    
    output = 
      self.b0 * output 
      + self.b1 * self.x2_1 
      + self.b2 * self.x2_2
      - self.a1 * self.y2_1
      - self.a2 * self.y2_2;

    self.x2_2 = self.x2_1;
    self.x2_1 = self.y1_1;
    self.y2_2 = self.y2_1;
    self.y2_1 = output;
    output
      
  }

  fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.a1 = coeffs.a1;
    self.a2 = coeffs.a2;
    self.b0 = coeffs.b0;
    self.b1 = coeffs.b1;
    self.b2 = coeffs.b2;
  }
}

impl BiquadTrait for Biquad8 {
  fn process(&mut self, sample: f32) -> f32 {
    let mut output = 
      self.b0 * sample 
      + self.b1 * self.x1_1 
      + self.b2 * self.x1_2
      - self.a1 * self.y1_1
      - self.a2 * self.y1_2;

    self.x1_2 = self.x1_1;
    self.x1_1 = sample;
    self.y1_2 = self.y1_1;
    self.y1_1 = output;
    
    output = self.b0 * output 
      + self.b1 * self.x2_1 
      + self.b2 * self.x2_2
      - self.a1 * self.y2_1
      - self.a2 * self.y2_2;

    self.x2_2 = self.x2_1;
    self.x2_1 = self.y1_1;
    self.y2_2 = self.y2_1;
    self.y2_1 = output;
    output
      
  }

  fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.a1 = coeffs.a1;
    self.a2 = coeffs.a2;
    self.b0 = coeffs.b0;
    self.b1 = coeffs.b1;
    self.b2 = coeffs.b2;
  }
}

#[derive(Clone, Copy)]
pub struct BiquadN<const N: usize> {
  bq: [Biquad; N],
  pub(super) a1: f32,
  pub(super) a2: f32,
  pub(super) b0: f32,
  pub(super) b1: f32,
  pub(super) b2: f32,
  prev_in: f32,
  prev_out: f32
}

#[allow(clippy::new_without_default)]
impl<const N: usize> BiquadN<N> {
  pub fn new() -> Self {
    Self {
      bq: [Biquad::new(); N],
      a1: 0.0,
      a2: 0.0,
      b0: 0.0,
      b1: 0.0,
      b2: 0.0,
      prev_in: 0.0,
      prev_out: 0.0
    }
  }
}

impl<const N:usize> BiquadTrait for BiquadN<N> {
  fn process(&mut self, sample: f32) -> f32 {
    let mut input = sample;
    for mut b in self.bq {
      input = b.process(input);
    }
    input 
  }
  
  fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.a1 = coeffs.a1;
    self.a2 = coeffs.a2;
    self.b0 = coeffs.b0;
    self.b1 = coeffs.b1;
    self.b2 = coeffs.b2;
  }
}
