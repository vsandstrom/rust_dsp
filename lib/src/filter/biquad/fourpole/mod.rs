use super::{BiquadCoeffs, BiquadTrait};

#[derive(Clone, Copy)]
pub struct Biquad4 {
  x1_1: f32, x1_2: f32, y1_1: f32, y1_2: f32,
  x2_1: f32, x2_2: f32, y2_1: f32, y2_2: f32,
  bq: BiquadCoeffs,
}

impl Biquad4 {
  pub fn new() -> Self {
    Self { 
      x1_1: 0.0, x1_2: 0.0, y1_1: 0.0, y1_2: 0.0, x2_1: 0.0, x2_2: 0.0, y2_1: 0.0, y2_2: 0.0,
      bq: BiquadCoeffs{a1: 0.0, a2: 0.0, b0: 0.0, b1: 0.0, b2: 0.0},
    }
  }
}

impl Default for Biquad4 {
  fn default() -> Self { Self::new() }
}

impl BiquadTrait for Biquad4 {
  fn process(&mut self, sample: f32) -> f32 {
    let mut output = 
        self.bq.b0 * sample 
      + self.bq.b1 * self.x1_1 
      + self.bq.b2 * self.x1_2
      - self.bq.a1 * self.y1_1
      - self.bq.a2 * self.y1_2;

    self.x1_2 = self.x1_1;
    self.x1_1 = sample;
    self.y1_2 = self.y1_1;
    self.y1_1 = output;
    
    output = 
        self.bq.b0 * output 
      + self.bq.b1 * self.x2_1 
      + self.bq.b2 * self.x2_2
      - self.bq.a1 * self.y2_1
      - self.bq.a2 * self.y2_2;

    self.x2_2 = self.x2_1;
    self.x2_1 = self.y1_1;
    self.y2_2 = self.y2_1;
    self.y2_1 = output;
    output
      
  }

  fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
    self.bq = coeffs;
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
