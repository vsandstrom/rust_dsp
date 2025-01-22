use crate::wavetable::shared::Wavetable;
use crate::interpolation::{Cubic, Linear};
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
pub unsafe extern "C" fn wavetable_play_linear(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut Wavetable)).play::<Linear>(table, frequency, phase)
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_play_cubic(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut Wavetable)).play::<Cubic>(table, frequency, phase)
}

