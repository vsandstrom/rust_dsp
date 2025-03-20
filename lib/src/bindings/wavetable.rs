use crate::wavetable::shared::Wavetable;
use crate::interpolation::{Cubic, Floor, Linear};
use alloc::{slice, boxed::Box};

#[repr(C)]
pub struct WavetableOpaque;
/// Underlying structure:
/// ```ignore
/// pub struct Wavetable {
///   position: f32,
///   samplerate: f32,
///   sr_recip: f32,
/// }
/// ```

#[no_mangle]
/// Constructor
pub extern "C" fn wavetable_new() -> *mut WavetableOpaque {
  let w = Box::new(Wavetable::new());
  Box::into_raw(w) as *mut WavetableOpaque
}


#[no_mangle]
/// Destructor
pub extern "C" fn wavetable_delete(wavetable: *mut WavetableOpaque) {
  if !wavetable.is_null() {
    unsafe {drop(Box::from_raw(wavetable as *mut Wavetable))}
  }
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_set_samplerate(wavetable: *mut WavetableOpaque, samplerate: f32) {
  (*(wavetable as *mut Wavetable)).set_samplerate(samplerate)
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_play_floor(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut Wavetable)).play::<Floor>(table, frequency, phase)
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_play_linear(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut Wavetable)).play::<Linear>(table, frequency, phase)
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_play_cubic(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut Wavetable)).play::<Cubic>(table, frequency, phase)
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
      WavetableOpaque,
      wavetable_play_cubic,
    }, 
    waveshape::traits::Waveshape, 
  };

  const SAMPLERATE: f32 = 48000.0;

  #[test] 
  fn triangletest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let wt: *mut WavetableOpaque = wavetable_new();
    unsafe { wavetable_set_samplerate(wt, SAMPLERATE); }
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      unsafe{
        let out = wavetable_play_floor(wt, table.as_ptr(), table.len(), SAMPLERATE/ SIZE as f32,  0.0);
        shape.push(out);
      }
    }
    wavetable_delete(wt);
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape);
  }
  
  #[test] 
  fn interptest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let wt: *mut WavetableOpaque = wavetable_new();
    unsafe {wavetable_set_samplerate(wt, SAMPLERATE);}
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      unsafe {
        let out = wavetable_play_linear(wt, table.as_ptr(), SIZE, SAMPLERATE / SIZE as f32, 1.0);
        shape.push(out);
      }
    }
    wavetable_delete(wt);
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
    let wt: *mut WavetableOpaque = wavetable_new();
    unsafe {wavetable_set_samplerate(wt, SAMPLERATE);}
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(
        unsafe{
          wavetable_play_linear(wt, table.as_ptr(), SIZE, SAMPLERATE / SIZE as f32 * 0.5, 0.0)
        }
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
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let wt: *mut WavetableOpaque = wavetable_new();
    unsafe {wavetable_set_samplerate(wt, SAMPLERATE);}
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(
        unsafe{
          wavetable_play_cubic(wt, table.as_ptr(), SIZE, SAMPLERATE / SIZE as f32 * 0.5, 0.0)
        }
      );
    }
    wavetable_delete(wt);
    assert_eq!(vec![
       0.75,  1.0,  0.75, 0.0,
      -0.75, -1.0, -0.75, 0.0
    ], shape);
  }
}

