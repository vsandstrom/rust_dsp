use crate::{delay::{Delay, FixedDelay, DelayTrait}, interpolation::{Linear, Cubic}};

#[repr(C)]
// ```
// Underlying structure:
/// struct Delay {
///   buffer: Vec<f32>,
///   delay: f32,
///   position: usize,
/// }
// ```
pub struct DelayOpaque;

#[no_mangle]
/// Constructor
pub extern "C" fn delay_new(length: usize) -> *mut DelayOpaque {
  Box::into_raw(Box::new(Delay::new(length))) as *mut DelayOpaque
}


#[no_mangle]
/// Destructor
pub unsafe extern "C" fn delay_delete(delay: *mut DelayOpaque) {
  if !delay.is_null() {
    drop(Box::from_raw(delay as *mut Delay))
  }
}

#[no_mangle]
pub unsafe extern "C" fn delay_play_linear(delay: *mut DelayOpaque, input: f32, seconds: f32, feedback: f32) -> f32 {
  (*(delay as *mut Delay)).play::<Linear>(input, seconds, feedback)
}

#[no_mangle]
pub unsafe extern "C" fn delay_play_cubic(delay: *mut DelayOpaque, input: f32, seconds: f32, feedback: f32) -> f32 {
  (*(delay as *mut Delay)).play::<Cubic>(input, seconds, feedback)
}




// #[repr(C)]
// pub struct FixedDelayOpaque;
//
// #[no_mangle]
// /// Constructor
// pub extern "C" fn fixed_delay_new(length: usize) -> *mut FixedDelayOpaque {
//   Box::into_raw(Box::new(FixedDelay::new(length))) as *mut FixedDelayOpaque
// }
//
//
// #[no_mangle]
// /// Destructor
// pub unsafe extern "C" fn fixed_delay_delete(delay: *mut FixedDelayOpaque) {
//   if !delay.is_null() {
//     drop(Box::from_raw(delay as *mut FixedDelay))
//   }
// }
