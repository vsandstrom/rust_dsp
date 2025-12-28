use crate::{
  grains::{GrainTrait, dynamic::Granulator}, interpolation::{Cubic, Linear}, waveshape::hanning
};
use alloc::boxed::Box;

#[repr(C)]
/// /* Underlying Structure */
/// ```ignore
/// pub struct Granulator {
///   buffer: Vec<f32>,
///   buf_size: usize,
///   envelope: Vec<f32>,
///   env_size: usize,
///   rec_pos: usize,
///   pub recording: bool,
///   next_grain: usize,
///   grains: Vec<Grain>,
///   samplerate: u32,
///   sr_recip: f32,
/// }
/// ```
pub struct GranulatorRust;

#[unsafe(no_mangle)]
/// Constructor
pub extern "C" fn granulator_new(samplerate: u32, num_grains: usize, buf_size: usize) -> *mut GranulatorRust {
  let mut table = [0.0; 1024];
  hanning(&mut table);
  let shape = table.to_vec();
  let g = Box::new(Granulator::new(shape, samplerate, num_grains, buf_size));
  Box::into_raw(g) as *mut GranulatorRust
}

#[unsafe(no_mangle)]
/// Destructor
pub extern "C" fn granulator_delete(granulator: *mut GranulatorRust) {
  if !granulator.is_null() {
    unsafe {drop(Box::from_raw(granulator as *mut Granulator))}
  }
}

#[unsafe(no_mangle)]
/// Trigger new grain
pub extern "C" fn granulator_trigger(granulator: *mut GranulatorRust, position: f32, duration: f32, rate: f32, jitter: f32) -> bool {
  unsafe {(*(granulator as *mut Granulator)).trigger_new(position, duration, rate, jitter)}
}

#[unsafe(no_mangle)]
/// Play with linear buffer interpolation
pub extern "C" fn granulator_play_linear(granulator: *mut GranulatorRust) -> f32 {
  unsafe {(*(granulator as *mut Granulator)).play::<Linear, Linear>() }
}

#[unsafe(no_mangle)]
/// Play with cubic buffer interpolation
pub extern "C" fn granulator_play_cubic(granulator: *mut GranulatorRust) -> f32 {
  unsafe {(*(granulator as *mut Granulator)).play::<Cubic, Linear>() }
}

#[unsafe(no_mangle)]
/// Record into buffer
pub extern "C" fn granulator_record(granulator: *mut GranulatorRust, sample: f32) -> bool {
  unsafe {(*(granulator as *mut Granulator)).record(sample) }.is_some()
}


