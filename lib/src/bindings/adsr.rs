use crate::adsr::{ADSREnvelope, Reset};
use alloc::boxed::Box;

#[repr(C)]
/// ```ignore
/// // Underlying structure:
/// #[derive(Debug, Copy, Clone)]
/// pub struct ADSREnvelope {
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
pub struct ADSREnvelopeRust;

#[unsafe(no_mangle)]
/// Constructor
pub unsafe extern "C" fn adsr_new(samplerate: u32) -> *mut ADSREnvelopeRust {
  Box::into_raw(Box::new(ADSREnvelope::new(samplerate))) as *mut ADSREnvelopeRust
}

#[unsafe(no_mangle)]
/// Destructor
pub unsafe extern "C" fn adsr_delete(adsr: *mut ADSREnvelopeRust) {
  if !adsr.is_null() {
    drop(Box::from_raw(adsr as *mut ADSREnvelope))
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_attack_val (adsr: *mut ADSREnvelopeRust, atk_value: f32) {
  (*(adsr as *mut ADSREnvelope)).set_attack_val(atk_value);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_attack_dur (adsr: *mut ADSREnvelopeRust, atk_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_attack_dur(atk_duration); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_attack_cur (adsr: *mut ADSREnvelopeRust, atk_curve: f32) { 
  (*(adsr as *mut ADSREnvelope)).set_attack_cur(atk_curve); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_decay_dur  (adsr: *mut ADSREnvelopeRust, dec_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_decay_dur(dec_duration); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_decay_cur  (adsr: *mut ADSREnvelopeRust, dec_curve: f32) {
  (*(adsr as *mut ADSREnvelope)).set_decay_cur(dec_curve); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_sustain_val(adsr: *mut ADSREnvelopeRust, sus_value: f32) {
  (*(adsr as *mut ADSREnvelope)).set_sustain_val(sus_value); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_release_dur(adsr: *mut ADSREnvelopeRust, rel_duration: f32) {
  (*(adsr as *mut ADSREnvelope)).set_release_dur(rel_duration); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_release_cur(adsr: *mut ADSREnvelopeRust, rel_curve: f32) {
  (*(adsr as *mut ADSREnvelope)).set_release_cur(rel_curve); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_set_reset_type (adsr: *mut ADSREnvelopeRust, reset: Reset) {
  (unsafe {*(adsr as *mut ADSREnvelope) }).set_reset_type(reset); 
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adsr_play(adsr: *mut ADSREnvelopeRust, trig: bool, sustain: bool) -> f32 {
  (*(adsr as *mut ADSREnvelope)).play(sustain)
}

