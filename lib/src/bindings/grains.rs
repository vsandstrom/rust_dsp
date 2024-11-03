use crate::{
  envelope::{EnvType, Envelope}, 
  interpolation::{Linear, Interpolation}, 
  waveshape::hanning,
  grains::Granulator
};

#[no_mangle]
/// Constructor
pub extern "C" fn granulator_new(samplerate: f32, num_grains: usize, buf_size: usize) -> *mut GranulatorOpaque {
  let shape = hanning(&mut [0.0; 1024]).to_vec();
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
/// Play 
pub extern "C" fn granulator_play(granulator: *mut GranulatorOpaque) -> f32 {
  unsafe {(*(granulator as *mut Granulator)).play::<Linear, Linear>() }
}


#[repr(C)]
pub struct GranulatorOpaque;

