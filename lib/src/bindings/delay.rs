use crate::{delay::Delay, interpolation::{linear::Linear, cubic::Cubic}};
use alloc::boxed::Box;
use core::slice::from_raw_parts_mut;

/// Underlying structure:
/// ```ignore
/// struct Delay {
///   buffer: Vec<f32>,
///   delay: f32,
///   position: usize,
/// }
/// ```
#[repr(C)]
pub struct DelayRust;

#[unsafe(no_mangle)]
/// Constructor
pub extern "C" fn delay_new() -> *mut DelayRust {
  Box::into_raw(Box::new(Delay::new())) as *mut DelayRust
}


#[unsafe(no_mangle)]
/// Destructor
pub extern "C" fn delay_delete(delay: *mut DelayRust) {
  if !delay.is_null() {
    unsafe {
      drop(Box::from_raw(delay as *mut Delay))
    }
  }
}

// #[unsafe(no_mangle)]
// pub extern "C" fn delay_play_linear(delay: *mut DelayRust, buffer: *mut f32, buf_len: usize, input: f32, seconds: f32, feedback: f32) -> f32 {
//   unsafe {
//     let buffer = from_raw_parts_mut(buffer, buf_len);
//     (*(delay as *mut Delay)).play::<Linear>(buffer, input, seconds, feedback)
//   }
// }
//
// #[unsafe(no_mangle)]
// pub extern "C" fn delay_play_cubic(delay: *mut DelayRust, buffer: *mut f32, buf_len: usize, input: f32, seconds: f32, feedback: f32) -> f32 {
//   unsafe {
//     let buffer = from_raw_parts_mut(buffer, buf_len);
//     (*(delay as *mut Delay)).play::<Cubic>(buffer, input, seconds, feedback)
//   }
// }




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
