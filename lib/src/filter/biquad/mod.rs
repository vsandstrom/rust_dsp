pub mod twopole;
pub mod fourpole;
pub mod eightpole;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BiquadCoeffs {a1: f32, a2: f32, b0: f32, b1: f32, b2: f32}
// pub trait BiquadTrait {
//   fn process(&mut self, sample: f32) -> f32;
//   fn set_coeffs(&mut self, coeffs: BiquadCoeffs);
//   fn calc_lpf(&mut self, w: f32, q: f32);
//   fn calc_hpf(&mut self, w: f32, q: f32);
//   fn calc_bpf(&mut self, w: f32, q: f32);
//   fn calc_peq(&mut self, w: f32, q: f32, gain: f32);
//   fn calc_notch(&mut self, w: f32, q: f32);
// }


pub mod calc {
  use super::BiquadCoeffs;
  #[inline]
  pub fn calc_lpf(w: f32, q: f32) -> BiquadCoeffs {
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
  pub fn calc_bpf(w: f32, q: f32) -> BiquadCoeffs {
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
  pub fn calc_hpf(w: f32, q: f32) -> BiquadCoeffs {
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
  pub fn calc_peq(w: f32, q: f32, gain: f32) -> BiquadCoeffs {
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
}
