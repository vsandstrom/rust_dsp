use crate::adsr::{ADSREnvelope, Reset};
use alloc::boxed::Box;

#[repr(C)]
/// ```ignore
/// Underlying structure:
/// #[derive(Debug)]
/// struct ADSREnvelope {
///   atk_value: f32,
///   atk_duration: f32,
///   atk_curve: f32,
///
///   dec_duration: f32,
///   dec_curve: f32,
///
///   sus_value: f32,
///
///   rel_duration: f32,
///   rel_curve: f32,
///
///   stage: EnvStage,
///   start: f32,
///   prev: f32,
///   next: f32,
///   playing: bool,
///   reset: Reset,
///   count: usize,
///   sr: f32
/// }
/// ```
pub struct ADSREnvelopeOpaque;

#[no_mangle]
/// Constructor
pub extern "C" fn adsr_new(samplerate: f32) -> *mut ADSREnvelopeOpaque {
  Box::into_raw(Box::new(ADSREnvelope::new(samplerate))) as *mut ADSREnvelopeOpaque
}

#[no_mangle]
/// Destructor
pub unsafe extern "C" fn adsr_delete(adsr: *mut ADSREnvelopeOpaque) {
  if !adsr.is_null() {
    drop(Box::from_raw(adsr as *mut ADSREnvelope))
  }
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_attack_val (adsr: *mut ADSREnvelopeOpaque, atk_value: f32) {
  (*(adsr as *mut ADSREnvelope)).set_attack_val(atk_value);
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_attack_dur (adsr: *mut ADSREnvelopeOpaque, atk_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_attack_dur(atk_duration); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_attack_cur (adsr: *mut ADSREnvelopeOpaque, atk_curve: f32) { 
  (*(adsr as *mut ADSREnvelope)).set_attack_cur(atk_curve); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_decay_dur  (adsr: *mut ADSREnvelopeOpaque, dec_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_decay_dur(dec_duration); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_decay_cur  (adsr: *mut ADSREnvelopeOpaque, dec_curve: f32) {
  (*(adsr as *mut ADSREnvelope)).set_decay_cur(dec_curve); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_sustain_val(adsr: *mut ADSREnvelopeOpaque, sus_value: f32) {
  (*(adsr as *mut ADSREnvelope)).set_sustain_val(sus_value); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_release_dur(adsr: *mut ADSREnvelopeOpaque, rel_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_release_dur(rel_duration); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_release_cur(adsr: *mut ADSREnvelopeOpaque, rel_curve: f32) {
  (*(adsr as *mut ADSREnvelope)).set_release_cur(rel_curve); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_set_reset_type (adsr: *mut ADSREnvelopeOpaque, reset: Reset) {
  (*(adsr as *mut ADSREnvelope)).set_reset_type(reset); 
}

#[no_mangle]
pub unsafe extern "C" fn adsr_play(adsr: *mut ADSREnvelopeOpaque, trig: bool, sustain: bool) -> f32 {
  (*(adsr as *mut ADSREnvelope)).play(trig, sustain)
}

