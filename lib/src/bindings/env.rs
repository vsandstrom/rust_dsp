use crate::envelope::new_env::{self, BreakPoint, Envelope};
use alloc::{slice, boxed::Box, vec};

/// Underlying structure:
/// ```ignore
/// #[derive(Clone)]
/// pub struct Envelope {
///   breakpoints: Vec<BreakPoint>,
///   counter: f32,
///   segment: usize,
///   steps: usize,
///   inc: f32,
///   previous_value: f32,
///   samplerate: f32,
///   rate: f32,
///   playing: bool,
///   looping: bool,
///   reset: Reset
/// }
/// ```
#[repr(C)]
pub struct EnvelopeRust;

#[repr(C)]
pub enum Reset {
  Hard,
  Soft
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_new(
  value: *const f32,
  v_len: usize,
  duration: *const f32,
  d_len: usize,
  curve: *const f32,
  c_len: usize,
  samplerate: f32,
) -> *mut EnvelopeRust {
  let v = unsafe {slice::from_raw_parts(value, v_len)};
  let d = unsafe {slice::from_raw_parts(duration, d_len)};
  let c = unsafe {slice::from_raw_parts(curve, c_len)};
  let mut x = vec!();
  for (value, (duration, curve)) in v.iter().zip(d.iter().zip(c.iter())) {
    x.push(BreakPoint{value: *value, duration: *duration, curve: Some(*curve)});
  }
  let env = Envelope::new(x, samplerate).unwrap();
  Box::into_raw(Box::new(env)) as *mut EnvelopeRust
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_delete(env: *mut EnvelopeRust) {
  if !env.is_null() { unsafe {drop(Box::from_raw(env as *mut Envelope))} }
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_trig(env: *mut EnvelopeRust) {
  unsafe {(*(env as *mut Envelope)).trig()}
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_play(env: *mut EnvelopeRust) -> f32 {
  unsafe {(*(env as *mut Envelope)).play()}
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_set_reset_type(env: *mut EnvelopeRust, reset_type: Reset) {
  unsafe {
    match reset_type {
      Reset::Hard => (*(env as *mut Envelope)).set_reset_type(new_env::Reset::HARD),
      Reset::Soft => (*(env as *mut Envelope)).set_reset_type(new_env::Reset::SOFT),
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn envelope_loopable(env: *mut EnvelopeRust, loopable: bool) {
  unsafe {(*(env as *mut Envelope)).set_loopable(loopable)}
}

