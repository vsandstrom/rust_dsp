use crate::envelope::new_env::{self, BreakPoint, Envelope};

#[repr(C)]
pub struct EnvelopeOpaque;

#[repr(C)]
pub enum Reset {
  Hard,
  Soft
}

#[no_mangle]
pub extern "C" fn envelope_new(
  value: *const f32,
  v_len: usize,
  duration: *const f32,
  d_len: usize,
  curve: *const f32,
  c_len: usize,
  samplerate: f32,
) -> *mut EnvelopeOpaque {
  let v = unsafe {std::slice::from_raw_parts(value, v_len)};
  let d = unsafe {std::slice::from_raw_parts(duration, d_len)};
  let c = unsafe {std::slice::from_raw_parts(curve, c_len)};
  let mut x = vec!();
  for (value, (duration, curve)) in v.iter().zip(d.iter().zip(c.iter())) {
    x.push(BreakPoint{value: *value, duration: *duration, curve: Some(*curve)});
  }
  let env = Envelope::new(x, samplerate).unwrap();
  Box::into_raw(Box::new(env)) as *mut EnvelopeOpaque
}

#[no_mangle]
pub extern "C" fn envelope_delete(env: *mut EnvelopeOpaque) {
  if !env.is_null() { unsafe {drop(Box::from_raw(env as *mut Envelope))} }
}

#[no_mangle]
pub extern "C" fn envelope_trig(env: *mut EnvelopeOpaque) {
  unsafe {(*(env as *mut Envelope)).trig()}
}

#[no_mangle]
pub extern "C" fn envelope_play(env: *mut EnvelopeOpaque) -> f32 {
  unsafe {(*(env as *mut Envelope)).play()}
}

#[no_mangle]
pub extern "C" fn envelope_set_reset_type(env: *mut EnvelopeOpaque, reset_type: Reset) {
  unsafe {
    match reset_type {
      Reset::Hard => (*(env as *mut Envelope)).set_reset_type(new_env::Reset::HARD),
      Reset::Soft => (*(env as *mut Envelope)).set_reset_type(new_env::Reset::SOFT),
    }
  }
}

#[no_mangle]
pub extern "C" fn envelope_loopable(env: *mut EnvelopeOpaque, loopable: bool) {
  unsafe {(*(env as *mut Envelope)).set_loopable(loopable)}
}

