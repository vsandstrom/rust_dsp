use crate::wavetable::shared::WaveTable;
use crate::interpolation::{Cubic, Linear};
use alloc::{slice, boxed::Box};

#[repr(C)]
pub struct WavetableOpaque;
// Underlying structure:
// pub struct WaveTable {
//   position: f32,
//   samplerate: f32,
//   sr_recip: f32,
// }
// ```

#[no_mangle]
/// Constructor
pub extern "C" fn wavetable_new() -> *mut WavetableOpaque {
  let w = Box::new(WaveTable::new());
  Box::into_raw(w) as *mut WavetableOpaque
}


#[no_mangle]
/// Destructor
pub extern "C" fn wavetable_delete(wavetable: *mut WavetableOpaque) {
  if !wavetable.is_null() {
    unsafe {drop(Box::from_raw(wavetable as *mut WaveTable))}
  }
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_set_samplerate(wavetable: *mut WavetableOpaque, samplerate: f32) {
  (*(wavetable as *mut WaveTable)).set_samplerate(samplerate)
}


#[no_mangle]
pub unsafe extern "C" fn wavetable_play_linear(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut WaveTable)).play::<Linear>(table, frequency, phase)
}

#[no_mangle]
pub unsafe extern "C" fn wavetable_play_cubic(wavetable: *mut WavetableOpaque, table: *const f32, table_length: usize, frequency: f32, phase: f32) -> f32 {
  let table = slice::from_raw_parts(table, table_length);
  (*(wavetable as *mut WaveTable)).play::<Cubic>(table, frequency, phase)
}

