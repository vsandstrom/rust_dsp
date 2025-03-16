use crate::filter::MultiModeTrait;
use super::BiquadCoeffs;

#[derive(Clone, Copy)]
pub struct Biquad {
  x1: f32, x2: f32, y1: f32, y2: f32,
  bq: BiquadCoeffs,
}

impl Default for Biquad {
  fn default() -> Self { Self::new() }
}

impl Biquad {
  pub fn new() -> Self {
    Self {
      x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, 
      bq: BiquadCoeffs{ a1: 0.0, a2: 0.0, b0: 0.0, b1: 0.0, b2: 0.0 },
    }
  }
  
  pub fn calc_next(&self, input: f32) -> f32 {
        self.bq.b0 * input 
      + self.bq.b1 * self.x1 
      + self.bq.b2 * self.x1
      - self.bq.a1 * self.y1
      - self.bq.a2 * self.y1
  }
  
  pub fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.bq = coeffs;
  }
}

impl MultiModeTrait for Biquad {
  // Direct form I
  fn process(&mut self, sample: f32) -> f32 {
    let output = {
        self.bq.b0 * sample 
      + self.bq.b1 * self.x1 
      + self.bq.b2 * self.x2
      - self.bq.a1 * self.y1
      - self.bq.a2 * self.y2
    };

    self.x2 = self.x1;
    self.x1 = sample;
    self.y2 = self.y1;
    self.y1 = output;
    output
  }

  #[inline]
  fn calc_lpf(&mut self, w: f32, q: f32) {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    self.bq.a1 = (-2.0 * w.cos()) / a0 ;
    self.bq.a2 = (1.0 - alpha) / a0;

    self.bq.b1 = (1.0 - w.cos()) / a0;
    self.bq.b0 = self.bq.b1 / 2.0 / a0;
    self.bq.b2 = self.bq.b0;
  }
    
  #[inline]
  fn calc_bpf(&mut self, w: f32, q: f32) {
    let alpha = w.sin() / (2.0 * q);
    
    let a0 = 1.0 + alpha;
    self.bq.a1 = (-2.0 * w.cos()) / a0;
    self.bq.a2 = (1.0 - alpha) / a0;
            
    self.bq.b0 = alpha / a0;
    self.bq.b1 = 0.0;
    self.bq.b2 = -alpha / a0;
  }

  #[inline]
  fn calc_hpf(&mut self, w: f32, q: f32) {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    self.bq.a1 = -2.0 * w.cos() / a0;
    self.bq.a2 = 1.0 - alpha / a0;
            
    self.bq.b0 = (1.0 + w.cos()) / 2.0 / a0;
    self.bq.b1 = -(self.bq.b0 * 2.0);
    self.bq.b2 = self.bq.b0;
  }

  #[inline]
   fn calc_notch(&mut self, w: f32, q: f32) {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    self.bq.a1 = -2.0 * w.cos() / a0;
    self.bq.a2 = (1.0 - alpha) / a0;
            
    self.bq.b0 = 1.0 / a0;
    self.bq.b1 = self.bq.a1;
    self.bq.b2 = self.bq.b0;
  }

  #[inline]
  fn calc_peq(&mut self, w: f32, q: f32, gain: f32) {
    let alpha = w.sin() / (2.0 * q);
    let a = f32::powf(10.0, gain/40.0);
    let a0 = (1.0 + alpha) / a;       //  1 + alpha
    self.bq.a1 = -2.0 * w.cos() / a0;  // -2 * cos(omega)
    self.bq.a2 = (1.0 - alpha) / a / a0;  //  1 - alpha / A
    self.bq.b0 = (1.0 + alpha) * a / a0;  // 1 + alpha * A
    self.bq.b1 = self.bq.a1;                    // -2 * cos(omega)
    self.bq.b2 = (1.0 - alpha) * a / a0;  // 1 - alpha * A 
  }
}
