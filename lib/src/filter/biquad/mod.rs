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
  HighShelf,
};


#[derive(Clone, Copy)]
#[repr(C)]
pub struct BiquadCoeffs {a1: f32, a2: f32, b0: f32, b1: f32, b2: f32}

pub trait BiquadTrait<T: BiquadKind> {
  fn update(&mut self, settings: &T::Settings);
}

#[repr(C)]
/// Contains the settings for the biquad filter. 
/// The `gain` parameter is ignored in a [`Biquad`](crate::filter::biquad::twopole::Biquad)
/// implementing [`Lpf`]/[`Bpf`]/[`Hpf`] and [`Notch`]
/// but used in [`Peq`], 
/// $$
/// omega = 2pi * freq / samplerate
///| Freq (Hz)  | Q   | Bandwidth (Hz) |
///| ---------- | --- | -------------- |
///| 1000       | 1   | 1000           |
///| 1000       | 2   | 500            |
///| 1000       | 10  | 100            |
///| 1000       | 0.5 | 2000           |
///$$
///
pub struct BiquadSettings {pub omega: f32, pub q: f32, pub gain: f32}

pub trait BiquadKind {
  type Settings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs;
}

// crate::impl_filter_kind!(
//   trait BiquadKind,
//   settings = BiquadSettings,
//   output = BiquadCoeffs,
//   mappings = {
//     Lpf => lpf,
//     Bpf => bpf,
//     Hpf => hpf,
//     Notch => notch,
//     Peq => peq [gain],
//     LowShelf => low_shelf [gain],
//     HighShelf => high_shelf [gain],
//   }
// );

impl BiquadKind for Lpf {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    BiquadCoeffs::lpf(settings.omega, settings.q)
  }
}

impl BiquadKind for Bpf {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    BiquadCoeffs::bpf(settings.omega, settings.q)
  }
}

impl BiquadKind for Hpf {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    BiquadCoeffs::hpf(settings.omega, settings.q)
  }
}

impl BiquadKind for Notch {
  type Settings = BiquadSettings;
  #[inline]
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    BiquadCoeffs::notch(settings.omega, settings.q)
  }
}

impl BiquadCoeffs {
#[inline]
  pub fn lpf(omega: f32, q: f32) -> Self {
    let alpha = omega.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * omega.cos()) / a0 ;
    let a2 = (1.0 - alpha) / a0;

    let b1 = (1.0 - omega.cos()) / a0;
    let b0 = b1 / 2.0 / a0;
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }
  
  #[inline]
  pub fn bpf(omega: f32, q: f32) -> Self {
    let alpha = omega.sin() / (2.0 * q);
    
    let a0 = 1.0 + alpha;
    let a1 = (-2.0 * omega.cos()) / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    Self{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn hpf(omega: f32, q: f32) -> Self {
    let alpha = omega.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * omega.cos() / a0;
    let a2 = 1.0 - alpha / a0;

    let b0 = (1.0 + omega.cos()) / 2.0 / a0;
    let b1 = -(b0 * 2.0);
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn notch(omega: f32, q: f32) -> Self {
    let alpha = omega.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * omega.cos() / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = 1.0 / a0;
    let b1 = a1;
    let b2 = b0;
    Self{a1, a2, b0, b1, b2}
  }

impl BiquadKind for Peq {
  type Settings = BiquadSettings;
  fn calc(settings: &Self::Settings) -> BiquadCoeffs {
    let alpha = settings.w.sin() / (2.0 * settings.q);
    let a = f32::powf(10.0, settings.gain/40.0);
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
}

