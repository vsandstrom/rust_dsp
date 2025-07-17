pub mod twopole;
pub mod fourpole;
pub mod eightpole;

use super::{
  Lpf,
  Bpf,
  Hpf,
  Peq,
  Notch,
  LowShelf,
  HighShelf
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BiquadCoeffs {a1: f32, a2: f32, b0: f32, b1: f32, b2: f32}

pub trait BiquadTrait<T: BiquadKind> {
  fn update(&mut self, settings: &T::Settings);
}

#[repr(C)]
pub struct BiquadSettings {pub w: f32, pub q: f32, gain: f32}

pub trait BiquadKind {
  type Settings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs;
}



impl BiquadKind for Lpf {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * settings.w.cos()) / a0 ;
    let a2 = (1.0 - alpha) / a0;

    let b1 = (1.0 - settings.w.cos()) / a0;
    let b0 = b1 / 2.0 / a0;
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }
}

impl BiquadKind for Bpf {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * settings.w.cos()) / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }
}

impl BiquadKind for Hpf {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * settings.w.cos() / a0;
    let a2 = 1.0 - alpha / a0;

    let b0 = (1.0 + settings.w.cos()) / 2.0 / a0;
    let b1 = -(b0 * 2.0);
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }
}

impl BiquadKind for Notch {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * settings.w.cos() / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = 1.0 / a0;
    let b1 = a1;
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }
}

impl BiquadKind for Peq {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let a = f32::powf(10.0, settings.gain/40.0);
    let a0 = (1.0 + alpha) / a;       //  1 + alpha
    let a1 = -2.0 * settings.w.cos() / a0;  // -2 * cos(omega)
    let a2 = (1.0 - alpha) / a / a0;  //  1 - alpha / A
    let b0 = (1.0 + alpha) * a / a0;  // 1 + alpha * A
    let b1 = a1;                    // -2 * cos(omega)
    let b2 = (1.0 - alpha) * a / a0;  // 1 - alpha * A
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }
}

impl BiquadKind for LowShelf {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let omega = settings.w.cos();
    let a = 10.0f32.powf(settings.gain / 40.0);
    
    let a_m1 = a - 1.0;
    let a_p1 = a + 1.0;
    let a_m1_omega = a_m1 * omega;
    let a_p1_omega = a_p1 * omega;
    let x = 2.0 * (a.sqrt() * alpha);

    if settings.gain >= 0.0 {
        // Boost Case
        let a0_inv = 1.0 / (a_p1 + a_m1_omega + x);
        let b0 = a * (a_p1 - a_m1_omega + x) * a0_inv;
        let b1 = 2.0 * a * (a_m1 - a_p1_omega) * a0_inv;
        let b2 = a * (a_p1 - a_m1_omega - x) * a0_inv;
        let a1 = -2.0 * (a_p1 + a_m1_omega) * a0_inv;
        let a2 = (a_p1 + a_m1_omega - x) * a0_inv;
        return BiquadCoeffs{a1,a2,b0,b1,b2};
    } 
    // Cut Case
    let a0_inv = 1.0 / (a_p1 - a_m1_omega + x);
    let b0 = a * (a_p1 + a_m1_omega + x) * a0_inv;
    let b1 = -2.0 * a * (a_m1 + a_p1_omega) * a0_inv;
    let b2 = a * (a_p1 + a_m1_omega - x) * a0_inv;
    let a1 = -2.0 * (a_p1 - a_m1_omega) * a0_inv;
    let a2 = (a_p1 - a_m1_omega - x) * a0_inv;
    BiquadCoeffs {a1,a2,b0,b1,b2}
  } 
}


impl BiquadKind for HighShelf {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let omega = settings.w.cos();
    let a = 10.0f32.powf(settings.gain / 40.0);
    
    let a_m1 = a - 1.0;
    let a_p1 = a + 1.0;
    let a_m1_omega = a_m1 * omega;
    let a_p1_omega = a_p1 * omega;
    let x = 2.0 * (a.sqrt() * alpha);

    if settings.gain >= 0.0 {
        // Boost
        let a0_inv = 1.0 / (a_p1 - a_m1_omega + x);
        let b0 = a * (a_p1 + a_m1_omega + x) * a0_inv;
        let b1 = -2.0 * a * (a_m1 + a_p1_omega) * a0_inv;
        let b2 = a * (a_p1 + a_m1_omega - x) * a0_inv;
        let a1 = -2.0 * (a_p1 - a_m1_omega) * a0_inv;
        let a2 = (a_p1 - a_m1_omega - x) * a0_inv;
        BiquadCoeffs {a1,a2,b0,b1,b2}
    } else {
        // Cut
        let a0_inv = 1.0 / (a_p1 + a_m1_omega + x);
        let b0 = a * (a_p1 - a_m1_omega + x) * a0_inv;
        let b1 = 2.0 * a * (a_m1 - a_p1_omega) * a0_inv;
        let b2 = a * (a_p1 - a_m1_omega - x) * a0_inv;
        let a1 = -2.0 * (a_p1 + a_m1_omega) * a0_inv;
        let a2 = (a_p1 + a_m1_omega - x) * a0_inv;
        BiquadCoeffs {a1,a2,b0,b1,b2}
    }
  }
}

