use crate::wavetable::shared::Wavetable;
use crate::interpolation::{cubic::Cubic, floor::Floor, linear::Linear};
use alloc::{slice, boxed::Box};

/// Underlying structure:
/// ```ignore
/// pub struct Wavetable {
///   position: f32,
///   samplerate: u32,
///   sr_recip: f32,
/// }
#[repr(C)]
pub struct WavetableRust;

#[unsafe(no_mangle)]
/// Constructor
pub extern "C" fn wavetable_new() -> *mut WavetableRust {
  let w = Box::new(Wavetable::new());
  Box::into_raw(w) as *mut WavetableRust
}


#[unsafe(no_mangle)]
/// Destructor
pub extern "C" fn wavetable_delete(wavetable: *mut WavetableRust) {
  if !wavetable.is_null() {
    unsafe {drop(Box::from_raw(wavetable as *mut Wavetable))}
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn wavetable_set_samplerate(wavetable: *mut WavetableRust, samplerate: u32) {
  unsafe {(*(wavetable as *mut Wavetable)).set_samplerate(samplerate)}
}

#[unsafe(no_mangle)]
pub extern "C" fn wavetable_play_floor(wavetable: *mut WavetableRust, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = unsafe { slice::from_raw_parts(table, table_length) };
  unsafe {(*(wavetable as *mut Wavetable)).play::<Floor>(table, frequency, phase)}
}

#[unsafe(no_mangle)]
pub extern "C" fn wavetable_play_linear(wavetable: *mut WavetableRust, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = unsafe { slice::from_raw_parts(table, table_length) };
  unsafe {(*(wavetable as *mut Wavetable)).play::<Linear>(table, frequency, phase)}
}

#[unsafe(no_mangle)]
pub extern "C" fn wavetable_play_cubic(wavetable: *mut WavetableRust, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = unsafe { slice::from_raw_parts(table, table_length) };
  unsafe {(*(wavetable as *mut Wavetable)).play::<Cubic>(table, frequency, phase)}
}


#[cfg(test)]
mod shared_table_tests {
  use alloc::vec;
  use crate::{
    bindings::wavetable::{
      wavetable_delete,
      wavetable_new, 
      wavetable_set_samplerate,
      wavetable_play_floor,
      wavetable_play_linear,
      WavetableRust,
    }, 
    waveshape::traits::Waveshape, 
  };

  const SAMPLERATE: u32 = 48000;
  const SIZE: usize = 16;
  const FREQ: f32 = SAMPLERATE as f32 / SIZE as f32;

  #[test] 
  fn triangletest_simple() {
    let table = [0.0; SIZE].triangle();
    let wt: *mut WavetableRust = wavetable_new();
    wavetable_set_samplerate(wt, SAMPLERATE); 
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wavetable_play_floor(wt, table.as_ptr(), table.len(), FREQ,  0.0);
      shape.push(out);
    }
    wavetable_delete(wt);
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape);
  }
  
  #[test] 
  fn interptest_simple() {
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let wt: *mut WavetableRust = wavetable_new();
    wavetable_set_samplerate(wt, SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wavetable_play_linear(wt, table.as_ptr(), SIZE, FREQ, 1.0);
      shape.push(out);
    }
    wavetable_delete(wt);
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

}

#[cfg(test)]
mod interpol_test {
  use alloc::vec;
  use crate::{
    bindings::wavetable::{
      wavetable_delete,
      wavetable_new, 
      wavetable_set_samplerate,
      wavetable_play_linear,
      WavetableRust,
      wavetable_play_cubic,
    }, 
    waveshape::traits::Waveshape, 
  };

  const SAMPLERATE: u32 = 48000;
  const SIZE: usize = 4;
  const FREQ: f32 = SAMPLERATE as f32 / SIZE as f32;
    #[test]
    fn linear_test_simple() {
      let dilude = 2;
      let mut table = [0.0; SIZE];
      let table = table.triangle();
      let wt: *mut WavetableRust = wavetable_new();
      wavetable_set_samplerate(wt, SAMPLERATE);
      let mut shape = vec!();
      for _ in 0..(SIZE * dilude) {
        shape.push(
          wavetable_play_linear(wt, table.as_ptr(), SIZE, FREQ * 0.5, 0.0)
        );
      }
      wavetable_delete(wt);
      assert_eq!(vec![
         0.5,  1.0,  0.5, 0.0,
        -0.5, -1.0, -0.5, 0.0
      ], shape);
    }
    
    #[test]
    fn cubic() {
      let dilude = 2;
      let mut table = [0.0; SIZE];
      let table = table.triangle();
      let wt: *mut WavetableRust = wavetable_new();
      wavetable_set_samplerate(wt, SAMPLERATE);
      let mut shape = vec!();
      for _ in 0..(SIZE * dilude) {
        shape.push(
          wavetable_play_cubic(wt, table.as_ptr(), SIZE, FREQ * 0.5, 0.0)
        );
      }
      wavetable_delete(wt);
      assert_eq!(vec![
         0.75,  1.0,  0.75, 0.0,
        -0.75, -1.0, -0.75, 0.0
      ], shape);
    }
}

