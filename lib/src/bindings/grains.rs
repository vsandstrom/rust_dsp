use crate::{
  envelope::{EnvType, Envelope}, 
  interpolation::{Linear, Interpolation}, 
  waveshape::hanning,
  grains::Granulator
};

const NUMGRAINS: usize = 32;
const BUFSIZE:   usize = {8*48000};


#[no_mangle]
pub extern "C" fn granulator_new(samplerate: f32) -> *mut GranulatorOpaque {
  let x = hanning(&mut [0.0; 1024]);
  let envtype: EnvType<0, 0> = EnvType::Vector(x.to_vec());
  let g = Box::new(Granulator::<NUMGRAINS, BUFSIZE>::new(&envtype, samplerate));
  Box::into_raw(g) as *mut GranulatorOpaque
}

#[no_mangle]
pub extern "C" fn granulator_delete(granulator: *mut GranulatorOpaque) {
  if !granulator.is_null() {
    unsafe {Box::from_raw(granulator as *mut Granulator<NUMGRAINS, BUFSIZE>);}
  }
}

#[no_mangle]
pub extern "C" fn granulator_trigger(granulator: *mut GranulatorOpaque, position: f32, duration: f32, rate: f32, jitter: f32) -> bool {
  let granulator = unsafe {&mut *(granulator as Granulator) };
  granulator.trigger_new(position, duration, rate, jitter)
  
}
#[no_mangle]
pub extern "C" fn granulator_play(granulator: *mut Granulator) -> f32 {
  Granulator::play::<Linear>(&granulator)
}


#[repr(C)]
pub struct GranulatorOpaque;

