pub mod twopole;
pub mod fourpole;
pub mod eightpole;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BiquadCoeffs {a1: f32, a2: f32, b0: f32, b1: f32, b2: f32}
pub trait BiquadTrait {
  fn update(&mut self, bq: BiquadCoeffs);
}
  
impl BiquadCoeffs {
  #[inline]
  pub fn lpf(w: f32, q: f32) -> Self {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * w.cos()) / a0 ;
    let a2 = (1.0 - alpha) / a0;

    let b1 = (1.0 - w.cos()) / a0;
    let b0 = b1 / 2.0 / a0;
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }
  
  #[inline]
  pub fn bpf(w: f32, q: f32) -> Self {
    let alpha = w.sin() / (2.0 * q);
    
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * w.cos()) / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    Self{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn hpf(w: f32, q: f32) -> Self {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * w.cos() / a0;
    let a2 = 1.0 - alpha / a0;

    let b0 = (1.0 + w.cos()) / 2.0 / a0;
    let b1 = -(b0 * 2.0);
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn notch(w: f32, q: f32) -> Self {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * w.cos() / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = 1.0 / a0;
    let b1 = a1;
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn peq(w: f32, q: f32, gain: f32) -> Self {
    let alpha = w.sin() / (2.0 * q);
    let a = f32::powf(10.0, gain/40.0);
    let a0 = (1.0 + alpha) / a;       //  1 + alpha
    let a1 = -2.0 * w.cos() / a0;  // -2 * cos(omega)
    let a2 = (1.0 - alpha) / a / a0;  //  1 - alpha / A
    let b0 = (1.0 + alpha) * a / a0;  // 1 + alpha * A
    let b1 = a1;                    // -2 * cos(omega)
    let b2 = (1.0 - alpha) * a / a0;  // 1 - alpha * A
    Self{a1, a2, b0, b1, b2}
  }
  // #[inline]
  // fn low_shelf(w: f32, q: f32, gain: f32) -> Self {
  //   todo!("not ready");
  //   let alpha = w.sin() / (2.0 * q);
  //   let omega = f32::cos(w);
  //   let a = f32::powf(10.0, gain/40.0);
  //   let a_m = a - 1.0; 
  //   let a_p = a + 1.0; 
  //   let a_p_o = a_m * omega;
  //   let a_m_o = a_p * omega;
  //   let x = 2.0 * f32::sqrt(a * alpha);
  //   let sign = if gain >= 0.0 { 1.0 } else { -1.0 };
  //
  //   let a0 =    1.0 / (a_p + sign * (-a_m_o + alpha + x));
  //   let a1 =    -2.0 * (a_p + sign *  -a_m_o)        * a0;
  //   let a2 =           (a_p + sign * (-a_m_o   - x)) * a0;  
  //   let b0 = a *       (a_p + sign * ( a_m_o   + x)) * a0;
  //   let b1 = a * 2.0 * (a_m + sign *   a_p_o)        * a0;
  //   let b2 = a *       (a_p + sign * ( a_m_o   - x)) * a0;
  //   Self{a1, a2, b0, b1, b2}
  // }
}



pub mod calc {
  use super::BiquadCoeffs;
  #[inline]
  pub fn lpf(w: f32, q: f32) -> BiquadCoeffs {
      let alpha = w.sin() / (2.0 * q);
      let a0 = 1.0 + alpha;
      let a1 = (-2.0 * w.cos()) / a0 ;
      let a2 = (1.0 - alpha) / a0;

      let b1 = (1.0 - w.cos()) / a0;
      let b0 = b1 / 2.0 / a0;
      let b2 = b0;
      BiquadCoeffs{a1, a2, b0, b1, b2}
  }
    
  #[inline]
  pub fn bpf(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * w.cos()) / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn hpf(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * w.cos() / a0;
    let a2 = 1.0 - alpha / a0;

    let b0 = (1.0 + w.cos()) / 2.0 / a0;
    let b1 = -(b0 * 2.0);
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn notch(w: f32, q: f32) -> BiquadCoeffs {
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
  pub fn peq(w: f32, q: f32, gain: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    let a = f32::powf(10.0, gain/40.0);
    let a0 = (1.0 + alpha) / a;       //  1 + alpha
    let a1 = -2.0 * w.cos() / a0;  // -2 * cos(omega)
    let a2 = (1.0 - alpha) / a / a0;  //  1 - alpha / A
    let b0 = (1.0 + alpha) * a / a0;  // 1 + alpha * A
    let b1 = a1;                    // -2 * cos(omega)
    let b2 = (1.0 - alpha) * a / a0;  // 1 - alpha * A
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  // #[inline]
  // fn low_shelf(w: f32, q: f32, gain: f32) -> BiquadCoeffs {
  //   let alpha = w.sin() / (2.0 * q);
  //   let omega = f32::cos(w);
  //   let a = f32::powf(10.0, gain/40.0);
  //   let a_m = a - 1.0; 
  //   let a_p = a + 1.0; 
  //   let a_p_o = a_m * omega;
  //   let a_m_o = a_p * omega;
  //   let x = 2.0 * f32::sqrt(a * alpha);
  //   let sign = if gain >= 0.0 { 1.0 } else { -1.0 };
  //
  //   let a0 =    1.0 / (a_p + sign * (-a_m_o + alpha + x));
  //   let a1 =    -2.0 * (a_p + sign *  -a_m_o)        * a0;
  //   let a2 =           (a_p + sign * (-a_m_o   - x)) * a0;  
  //   let b0 = a *       (a_p + sign * ( a_m_o   + x)) * a0;
  //   let b1 = a * 2.0 * (a_m + sign *   a_p_o)        * a0;
  //   let b2 = a *       (a_p + sign * ( a_m_o   - x)) * a0;
  //   BiquadCoeffs{a1, a2, b0, b1, b2}
  // }

  // #[inline]
  // fn calc_low_shelf(&mut self, w: f32, q: f32, gain: f32) {
  //   let alpha = w.sin() / (2.0 * q);
  //   let omega = f32::cos(w);
  //   let a = f32::powf(10.0, gain/40.0);
  //   let a_m = a - 1.0; 
  //   let a_p = a + 1.0; 
  //   let a_p_o = a_p * omega;
  //   let a_m_o = a_m * omega;
  //   let x = 2.0 * f32::sqrt(a * alpha);
  //   let sign = if gain >= 0.0 { 1.0 } else { -1.0 };
  //
  //   let a0 =    1.0 / (a_p - sign * (a_m_o - alpha - x));
  //   self.bq.a1 =    -2.0 * (a_p - sign *  a_m_o)        * a0;
  //   self.bq.a2 =           (a_p - sign * (a_m_o   + x)) * a0;  
  //   self.bq.b0 = a *       (a_p + sign * (a_m_o   + x)) * a0;
  //   self.bq.b1 = a * 2.0 * (a_m - sign *  a_p_o)        * a0 * sign;
  //   self.bq.b2 = a *       (a_p + sign * (a_m_o   - x)) * a0;
  // }
 // #[inline]
 //  fn calc_low_shelf(&mut self, w: f32, q: f32, gain: f32) {
 //    let alpha = w.sin() / (2.0 * q);
 //    let omega = f32::cos(w);
 //    let a = f32::powf(10.0, gain/40.0);
 //    let a_m = a - 1.0; 
 //    let a_p = a + 1.0; 
 //    let a_p_o = a_m * omega;
 //    let a_m_o = a_p * omega;
 //    let x = 2.0 * f32::sqrt(a * alpha);
 //
 //    let a0 =    1.0 / (a_p + a_m_o + alpha + x);
 //    self.bq.a1 =    -2.0 * (a_p + a_m_o)       * a0;
 //    self.bq.a2 =           (a_p + a_m_o   - x) * a0;  
 //    self.bq.b0 = a *       (a_p - a_m_o   + x) * a0;
 //    self.bq.b1 = a * 2.0 * (a_m - a_p_o)       * a0;
 //    self.bq.b2 = a *       (a_p - a_m_o   - x) * a0;
 //  }
}
