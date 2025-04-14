use crate::dsp::buffer::scale;
use alloc::{vec::Vec, borrow::ToOwned};
use core::f32::consts::{PI, TAU};

/// Create a complex waveform from amplitudes and phases of sine partials
/// (tip: normalize amplitudes to get waveform within -1.0 - 1.0)
pub fn complex_sine(table: &mut [f32], amps: &[f32], phases: &[f32]) {
  let len = table.len();
  let mut n: f32 = 1.0;
  if amps.len() == phases.len() {
    for i in 0..amps.len() {
      let inc = TAU * n / len as f32;
      let mut angle = inc * len as f32 * phases[i];
      for sample in table.iter_mut() {
        *sample += angle.sin() * amps[i];
        angle += inc;
      }
      n += 1.0;
    }
    scale(table, -1.0f32, 1.0f32);
  }
}
/// Sine: sin(2pi / table.len() * n)
pub fn sine(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = TAU / table.len() as f32;
  for sample in table.iter_mut() {
    *sample = angle.sin();
    angle += inc;
  }
}


/// Squared sinewave, positive bellcurve. Useful as envelope
pub fn hanning(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = PI / (table.len() as f32);
  for sample in table.iter_mut() {
    *sample = angle.sin().powf(2.0);
    angle += inc;
  }
}


/// Square
pub fn square(table: &mut [f32]) {
  let mut val = -1.0;
  let len = table.len();
  for (i, sample) in table.iter_mut().enumerate() {
    *sample = val;
    if i == len/2-1 { val = 1.0; } 
  }
}

/// Triangle 
pub fn triangle(table: &mut [f32]) {
  let mut angle = 0.0;
  let mut inc: f32 = 2.0 / (table.len() as f32 / 2.0);
  for sample in table.iter_mut() {
    if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
    *sample = angle;
    angle += inc;
  }
}

/// Sawtooth: -1.0 -> 1.0
pub fn sawtooth(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle - 1.0;
    angle += inc;
  }
}

/// Reverse sawtooth: 1.0 -> -1.0
pub fn reverse_sawtooth(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle + 1.0;
    angle -= inc;
  }
}
    
pub fn phasor(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 1.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle;
    angle += inc;
  }
}

pub mod traits {
use super::*;

pub trait Waveshape<const N: usize> {
  type Output;

  fn sine(&mut self) -> Self::Output;
  fn hanning(&mut self) -> Self::Output;
  fn triangle(&mut self) -> Self::Output;
  fn square(&mut self) -> Self::Output;
  fn sawtooth(&mut self) -> Self::Output;
  fn phasor(&mut self) -> Self::Output;
  fn reverse_sawtooth(&mut self) -> Self::Output;
  fn complex_sine<const M:usize>( 
    &mut self, amps: [f32; M], phases: [f32; M]
  ) -> Self::Output;
}

impl<const N: usize> Waveshape<N> for [f32; N] {
  type Output = [f32; N];

  fn hanning(&mut self) -> Self::Output{
    let mut angle: f32 = 0.0;
    let inc: f32 = PI / (self.len() as f32);
    for sample in self.iter_mut() {
      *sample = angle.sin().powf(2.0);
      angle += inc;
    }
    *self
  }

  /// Sine: sin(2pi / table.len() * n)
  fn sine(&mut self) -> Self::Output{
    let mut angle: f32 = 0.0;
    let inc: f32 = TAU / self.len() as f32;
    for sample in self.iter_mut() {
      *sample = angle.sin();
      angle += inc;
    }
    *self
  }

  ///Square
  fn square(&mut self) -> Self::Output{
    let mut val = -1.0;
    let len = self.len();
    for (i, sample) in self.iter_mut().enumerate() {
      *sample = val;
      if i == len/2-1 { val = 1.0; } 
    }
    *self
  }

  /// Triangle
  fn triangle(&mut self) -> Self::Output {
    let mut angle = 0.0;
    let mut inc: f32 = 2.0 / (self.len() as f32 / 2.0);
    for sample in self.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    *self
  }

  /// Sawtooth: -1.0 -> 1.0
  fn sawtooth(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle - 1.0;
      angle += inc;
    }
    *self
  }
  
  /// Sawtooth: -1.0 -> 1.0
  fn phasor(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 1.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle;
      angle += inc;
    }
    *self
  }

  /// Reverse sawtooth: 1.0 -> -1.0
  fn reverse_sawtooth(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle + 1.0;
      angle -= inc;
    }
    *self
  }

  /// Create a complex waveform from amplitudes and phases of sine partials
  fn complex_sine<const M:usize>(&mut self, amps: [f32; M], phases: [f32; M]) -> Self::Output {
    let mut n: f32 = 1.0;
    for (amp, phase) in amps.iter().zip(phases.iter()) {
      let inc = TAU * n / self.len() as f32;
      let mut angle = inc * self.len() as f32 * phase;
      for sample in self.iter_mut() {
        *sample += angle.sin() * amp;
        angle += inc;
      }
      n += 1.0;
    }
    scale(self, -1.0f32, 1.0f32);
    *self
  }
}


impl<const N:usize> Waveshape<N> for Vec<f32>  {
  type Output = Vec<f32>;
  /// Squared sinewave, positive bellcurve. Useful as envelope
  fn hanning(&mut self) -> Self::Output{
    let mut angle: f32 = 0.0;
    let inc: f32 = PI / (self.len() as f32);
    for sample in self.iter_mut() {
      *sample = angle.sin().powf(2.0);
      angle += inc;
    }
    self.to_owned()
  }
  
  /// Phasor: 0.0 -> 1.0
  /// Useful for looping through buffers
  fn phasor(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 1.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle;
      angle += inc;
    }
    self.to_owned()
  }

  /// Sine: sin(2pi / table.len() * n)
  fn sine(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = PI * 2.0 / self.len() as f32;
    for sample in self.iter_mut() {
      *sample = angle.sin();
      angle += inc;
    }
    self.to_owned()
  }

  /// Square
  fn square(&mut self) -> Self::Output {
    let mut val = -1.0;
    let len = self.len();
    for (i, sample) in self.iter_mut().enumerate() {
      *sample = val;
      if i == len/2 - 1 { val = 1.0; } 
    }
    self.to_owned()
  }

  /// Triangle
  fn triangle(&mut self) -> Self::Output {
    let mut angle = 0.0;
    let mut inc: f32 = 2.0 / (self.len() as f32 / 2.0);
    for sample in self.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    self.to_owned()
  }

  /// Sawtooth: 0.0 -> 1.0
  fn sawtooth(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle - 1.0;
      angle += inc;
    }
    self.to_owned()
  }
  

  /// Reverse sawtooth: 1.0 -> -1.0
  fn reverse_sawtooth(&mut self) -> Self::Output {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
    for sample in self.iter_mut() {
      *sample = angle + 1.0;
      angle -= inc;
    }
    self.to_owned()
  }

  /// Create a complex waveform from amplitudes and phases of sine partials
  fn complex_sine<const M: usize> (&mut self, amps: [f32; M], phases: [f32; M]) -> Self::Output {
    let mut n: f32 = 1.0;
    let len = self.len() as f32;
    for (amp, phase) in amps.iter().zip(phases.iter()) {
      let inc = PI * 2.0f32 * n / len;
      let mut angle = inc * len * phase;
      for sample in self.iter_mut() {
        *sample += angle.sin() * amp;
        angle += inc;
      }
      n += 1.0;
    }
    scale(self, -1.0f32, 1.0f32);
    self.to_owned()
  }
}
}

pub mod macros {
/// `sine!($size: literal)` - creates a compile-time fixed array of sine values.
/// `sine!($arr: expr)` - fills a mutable array with sine values in-place.
/// `sine![$default: literal; $size: literal]` - same as first, `$default` is unused (may be removed).
#[macro_export]
macro_rules! sine {
  ($size: literal)  => {{
    let inc: f32 = ::core::f32::consts::TAU / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin()
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let inc: f32 = ::core::f32::consts::TAU / arr.len() as f32;
    arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = (i as f32 * inc).sin()
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = ::core::f32::consts::TAU / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin()
    });
    x
  }};
}

#[macro_export]
macro_rules! hanning {
  ($size: literal)  => {{
    let inc: f32 = ::core::f32::consts::PI / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin().powf(2.0)
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let inc: f32 = ::core::f32::consts::PI / arr.len() as f32;
    arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = (i as f32 * inc).sin().powf(2.0)
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = ::core::f32::consts::PI / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin().powf(2.0)
    });
    x 
  }}
}

#[macro_export]
macro_rules! square {
  ($size: literal)  => {{
    let half = $size / 2;
    let x: [f32; $size] = std::array::from_fn(|i| {
      if i < half { -1.0f32 } else { 1.0f32 }
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let half = $arr.len() / 2;
    $arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = if i < half { -1.0f32 } else { 1.0f32 }
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let half = $size / 2;
    let arr: [f32; $size] = std::array::from_fn(|i| {
      if i < half { -1.0f32 } else { 1.0f32 }
    });
    arr
  }}
}

#[macro_export]
macro_rules! triangle {
  ($size: literal)  => {{
    let mut inc: f32 = 2.0 / ($size as f32 / 2.0);
    let mut angle = 0.0f32;
    let mut arr = [0.0f32; $size];
    for sample in arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    arr
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let mut inc: f32 = 2.0 / ($arr.len() as f32 / 2.0);
    let mut angle = 0.0f32;
    for sample in $arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let mut inc: f32 = 2.0 / ($size as f32 / 2.0);
    let mut angle = 0.0f32;
    let mut arr = [0.0f32; $size];
    for sample in arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    arr
  }}
}

#[macro_export]
macro_rules! sawtooth {
  ($size: literal)  => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32) - 1.0f32
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 2.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (inc * i as f32) - 1.0f32
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32) - 1.0f32
    });
    arr
  }}
}

#[macro_export]
macro_rules! reverse_sawtooth {
  ($size: literal)  => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (-inc * i as f32) + 1.0f32
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 2.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (-inc * i as f32) + 1.0f32
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (-inc * i as f32) + 1.0f32
    });
    arr
  }}
}

#[macro_export]
macro_rules! phasor {
  ($size: literal)  => {{
    let inc: f32 = 1.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32)
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 1.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (inc * i as f32)
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 1.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32)
    });
    arr
  }}
}

#[macro_export]
macro_rules! complex_sine {
  ($amps:expr, $phases:expr, $size:literal) => {{
    assert!($amps.len() == $phases.len(), "Amplitudes and phases mus be of equal length");
    assert!(!$amps.is_empty(), "Amplitudes and phases must be at least of length 1");
    let _: &[f32] = $amps;
    let _: &[f32] = $phases;
    let len = $size as f32;
    let mut arr = [0.0f32; $size];
    for (n, (a, p)) in $amps.iter().zip($phases.iter()).enumerate() {
      let inc = TAU * (n+1) as f32 / len as f32;
      let mut angle = inc * (len * p);
      arr.iter_mut().for_each(|sample| { *sample += angle.sin() * a; angle += inc; })
    }
    scale(&mut arr, -1.0, 1.0);
    arr
  }};
  ($amps: expr, $size: literal) => {{
    assert!(!$amps.is_empty(), "Amplitudes must be at least of length 1");
    let _: &[f32] = $amps;
    let len = $size as f32;
    let mut arr = [0.0f32; $size];
    for (n, a) in $amps.iter().enumerate() {
      let inc = TAU * (n+1) as f32 / len as f32;
      let mut angle = 0.0f32;
      arr.iter_mut().for_each(|sample| { *sample += angle.sin() * a; angle += inc; })
    }
    scale(&mut arr, -1.0, 1.0);
    arr

  }}
}
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::waveshape::traits::Waveshape;
  #[test]
  fn test_phasor() {
    let x = [0.0; 8].phasor().to_owned();
    assert_eq!(x[0], 0.0);
  }
  
  #[test]
  fn test_phasor2() {
    let x = [0.0; 8].phasor().to_owned();
    assert_eq!(x[7], 1.0);
  }
}
