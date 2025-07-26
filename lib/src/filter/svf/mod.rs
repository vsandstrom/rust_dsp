use super::Filter;
use super::{Lpf, Bpf, Hpf, Notch, Peq, HighShelf, LowShelf, FilterKind};
use core::marker::PhantomData;



pub trait SVFTrait<T: SVFKind> {
  fn update(&mut self, settings: &T::Settings);
}

pub struct SVFSettings{pub omega: f32, pub q: f32, pub gain: f32}

pub trait SVFKind {
  type Settings;
  fn calc(settings: &Self::Settings) -> SVFCoeffs;
}

// crate::impl_filter_kind!(
//   trait SVFKind, 
//   settings = SVFSettings,
//   output = SVFCoeffs,
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


impl SVFKind for Lpf {
  type Settings = SVFSettings;
  fn calc(settings: &Self::Settings) -> SVFCoeffs {
    SVFCoeffs::lpf(settings.omega, settings.q)
  }
}

impl SVFKind for Bpf {
  type Settings = SVFSettings;
  fn calc(settings: &Self::Settings) -> SVFCoeffs {
    SVFCoeffs::bpf(settings.omega, settings.q)
  }
}

impl SVFKind for Hpf {
  type Settings = SVFSettings;
  fn calc(settings: &Self::Settings) -> SVFCoeffs {
    SVFCoeffs::hpf(settings.omega, settings.q)
  }
}

impl SVFKind for Notch {
  type Settings = SVFSettings;
  fn calc(settings: &Self::Settings) -> SVFCoeffs {
    SVFCoeffs::notch(settings.omega, settings.q)
  }
}


pub struct SVFCoeffs { 
  a1: f32,
  a2: f32, 
  a3: f32,
  m0: f32,
  m1: f32,
  m2: f32,
  k:  f32
}

pub struct SVFilter<T: SVFKind> {
  ic1eq: f32,
  ic2eq: f32,
  c: SVFCoeffs,
  _marker: PhantomData<T>

}

impl<T: SVFKind> SVFilter<T> {
  pub fn new(settings: T::Settings) -> Self {
    Self {
      ic1eq: 0.0,
      ic2eq: 0.0,
      c: T::calc(&settings),
      _marker: PhantomData
    }
  }

  pub fn set_mode(&mut self, mode: f32) {
    todo!("moving between filter modes is not yet implemented")
  }
}

impl<T: SVFKind> Filter for SVFilter<T> {
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

impl<T: SVFKind> SVFTrait<T> for SVFilter<T> {
  fn update(&mut self, settings: &T::Settings) {
      self.c = T::calc(settings);
  }
}

impl SVFCoeffs {
  #[inline]
  pub fn lpf(omega: f32, q: f32) -> Self {
    let g = f32::tan(omega/2.0);
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
  pub fn bpf(omega: f32, q: f32) -> Self {
    let g = f32::tan(omega/2.0);
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
  pub fn hpf(omega: f32, q: f32) -> Self {
    let g = f32::tan(omega/2.0);
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
  pub fn notch(omega: f32, q: f32) -> Self {
    let g = f32::tan(omega/2.0);
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
