use crate::{
  grains::{GrainTrait, Granulator}, interpolation::{Cubic, Linear}, waveshape::hanning
};

#[repr(C)]
// /* Underlying Structure */
// pub struct Granulator {
//   buffer: Vec<f32>,
//   buf_size: usize,
//   envelope: Vec<f32>,
//   env_size: usize,
//   rec_pos: usize,
//   pub recording: bool,
//   next_grain: usize,
//   grains: Vec<Grain>,
//   samplerate: f32,
//   sr_recip: f32,
// }
pub struct GranulatorOpaque;

#[no_mangle]
/// Constructor
pub extern "C" fn granulator_new(samplerate: f32, num_grains: usize, buf_size: usize) -> *mut GranulatorOpaque {
  let mut table = [0.0; 1024];
  hanning(&mut table);
  let shape = table.to_vec();
  let g = Box::new(Granulator::new(shape, samplerate, num_grains, buf_size));
  Box::into_raw(g) as *mut GranulatorOpaque
}

#[no_mangle]
/// Destructor
pub extern "C" fn granulator_delete(granulator: *mut GranulatorOpaque) {
  if !granulator.is_null() {
    unsafe {drop(Box::from_raw(granulator as *mut Granulator))}
  }
}

#[no_mangle]
/// Trigger new grain
pub extern "C" fn granulator_trigger(granulator: *mut GranulatorOpaque, position: f32, duration: f32, rate: f32, jitter: f32) -> bool {
  unsafe {(*(granulator as *mut Granulator)).trigger_new(position, duration, rate, jitter)}
}

#[no_mangle]
/// Play with linear buffer interpolation
pub extern "C" fn granulator_play_linear(granulator: *mut GranulatorOpaque) -> f32 {
  unsafe {(*(granulator as *mut Granulator)).play::<Linear, Linear>() }
}

#[no_mangle]
/// Play with cubic buffer interpolation
pub extern "C" fn granulator_play_cubic(granulator: *mut GranulatorOpaque) -> f32 {
  unsafe {(*(granulator as *mut Granulator)).play::<Cubic, Linear>() }
}

#[no_mangle]
/// Record into buffer
pub extern "C" fn granulator_record(granulator: *mut GranulatorOpaque, sample: f32) -> bool {
  unsafe {(*(granulator as *mut Granulator)).record(sample) }.is_some()
}


