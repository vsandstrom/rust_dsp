use crate::interpolation::Interpolation;
use core::debug_assert;
use alloc::vec::Vec;

pub mod owned;
pub mod shared;
#[cfg(feature="std")]
pub mod arc;

#[cfg(test)]
mod shared_table_tests {
  use alloc::vec;
  use crate::{
    interpolation::{Floor, Linear},
    waveshape::traits::Waveshape,
  };

  use super::shared::Wavetable;

  const SAMPLERATE: u32 = 48000;
  const SIZE: usize = 16;
  const FREQ: f32 = SAMPLERATE as f32 / SIZE as f32;

  #[test] 
  fn triangletest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    // let mut wt = simple::Wavetable::new();
    let mut wt = Wavetable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(&table, FREQ, 0.0);
      shape.push(out); }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Linear>(&table, FREQ, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn linear_test_simple() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<Linear>(&table, FREQ * 2.0, 0.0));
    }
    // println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}

#[cfg(test)]
mod owned_table_tests {
  use alloc::vec;
  use crate::{
    interpolation::{Floor, Linear},
    waveshape::traits::Waveshape,
  };

  use super::owned::Wavetable;

  const SAMPLERATE: u32 = 48000;
  const SIZE: usize = 16;
  const FREQ: f32 = SAMPLERATE as f32 / SIZE as f32;

  #[test] 
  fn triangletest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    // let mut wt = simple::Wavetable::new();
    let mut wt = Wavetable::new(&table, SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(FREQ, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new(&table, SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Linear>(FREQ, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn linear_test_simple() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new(&table, SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<Linear>(FREQ * 2.0, 0.0));
    }
    // println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}


#[cfg(feature="std")]
#[cfg(test)]
mod arc_table_tests {
  use alloc::vec;
  use std::sync::{Arc, RwLock};
  use crate::{
    interpolation::{Floor, Linear},
    waveshape::traits::Waveshape,
  };

  use super::arc::Wavetable;

  const SAMPLERATE: u32 = 48000;
  const SIZE: usize = 16;
  const FREQ: f32 = SAMPLERATE as f32 / SIZE as f32;

  #[test] 
  fn triangletest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    // let mut wt = simple::Wavetable::new();
    let mut wt = Wavetable::new(Arc::new(RwLock::new(table.into())), SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(FREQ, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new(Arc::new(RwLock::new(table.into())), SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Linear>(FREQ, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn linear_test_simple() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = Wavetable::new(Arc::new(RwLock::new(table.into())), SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<Linear>( FREQ * 2.0, 0.0));
    }
    // println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}
