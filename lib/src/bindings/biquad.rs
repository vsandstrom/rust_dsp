use crate::filter::{biquad::{eightpole::Biquad8, fourpole::Biquad4, twopole::Biquad, BiquadCoeffs, BiquadTrait
}, Filter};
  
#[repr(C)]
pub struct BiquadOpaque;

#[no_mangle]
/// Constructor
pub extern "C" fn biquad_new() -> *mut BiquadOpaque {
  let bq = Box::new(Biquad::new());
  Box::into_raw(bq) as *mut BiquadOpaque
}

#[no_mangle]
/// Destructor
pub extern "C" fn biquad_delete(biquad: *mut BiquadOpaque) {
  if !biquad.is_null() {
    unsafe {drop(Box::from_raw(biquad as *mut Biquad))}
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad_process(biquad: *mut BiquadOpaque, sample: f32) -> f32 {
  (*(biquad as *mut Biquad)).process(sample)
}

#[no_mangle]
pub unsafe extern "C" fn biquad_update(biquad: *mut BiquadOpaque, coeffs: BiquadCoeffs) {
  (*(biquad as *mut Biquad)).update(coeffs);
}

#[no_mangle]
pub unsafe extern "C" fn biquad_calc_lpf(biquad: *mut BiquadOpaque, w: f32, q: f32) {
 (*(biquad as *mut Biquad)).update(BiquadCoeffs::lpf(w, q))
}
    
#[no_mangle]
pub unsafe extern "C" fn biquad_calc_bpf(biquad: *mut BiquadOpaque, w: f32, q: f32) {
 (*(biquad as *mut Biquad)).update(BiquadCoeffs::bpf(w, q))
}

#[no_mangle]
pub unsafe extern "C" fn biquad_calc_hpf(biquad: *mut BiquadOpaque, w: f32, q: f32) {
 (*(biquad as *mut Biquad)).update(BiquadCoeffs::hpf(w, q))
}

#[no_mangle]
pub unsafe extern "C" fn biquad_calc_notch(biquad: *mut BiquadOpaque, w: f32, q: f32) {
 (*(biquad as *mut Biquad)).update(BiquadCoeffs::notch(w, q))
}

#[no_mangle]
pub unsafe extern "C" fn biquad_calc_peq(biquad: *mut BiquadOpaque, w: f32, q: f32, gain: f32) {
 (*(biquad as *mut Biquad)).update(BiquadCoeffs::peq(w, q, gain))
}


// Biquad 4 pole


#[repr(C)]
pub struct Biquad4Opaque;

#[no_mangle]
/// Constructor
pub extern "C" fn biquad4_new() -> *mut Biquad4Opaque {
  let bq4 = Box::new(Biquad4::new());
  Box::into_raw(bq4) as *mut Biquad4Opaque
}

#[no_mangle]
/// Destructor
pub extern "C" fn biquad4_delete(biquad4: *mut Biquad4Opaque) {
  if !biquad4.is_null() {
    unsafe {drop(Box::from_raw(biquad4 as *mut Biquad4))}
  }
}


#[no_mangle]
pub unsafe extern "C" fn biquad4_process(biquad4: *mut Biquad4Opaque, sample: f32) -> f32 {
  (*(biquad4 as *mut Biquad4)).process(sample)
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_update(biquad4: *mut Biquad4Opaque, coeffs: BiquadCoeffs) {
  (*(biquad4 as *mut Biquad4)).update(coeffs);
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_calc_lpf(biquad4: *mut Biquad4Opaque, w: f32, q: f32) {
  (*(biquad4 as *mut Biquad4)).update(BiquadCoeffs::lpf(w, q));
}
    
#[no_mangle]
pub unsafe extern "C" fn biquad4_calc_bpf(biquad4: *mut Biquad4Opaque, w: f32, q: f32) {
  (*(biquad4 as *mut Biquad4)).update(BiquadCoeffs::bpf(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_calc_hpf(biquad4: *mut Biquad4Opaque, w: f32, q: f32) {
  (*(biquad4 as *mut Biquad4)).update(BiquadCoeffs::hpf(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_calc_notch(biquad4: *mut Biquad4Opaque, w: f32, q: f32) {
  (*(biquad4 as *mut Biquad4)).update(BiquadCoeffs::notch(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_calc_peq(biquad4: *mut Biquad4Opaque, w: f32, q: f32, gain: f32) {
  (*(biquad4 as *mut Biquad4)).update(BiquadCoeffs::peq(w, q, gain));
}

// Biquad 8 pole

#[repr(C)]
pub struct Biquad8Opaque;


#[no_mangle]
/// Constructor
pub extern "C" fn biquad8_new() -> *mut Biquad8Opaque {
  let bq8 = Box::new(Biquad8::new());
  Box::into_raw(bq8) as *mut Biquad8Opaque
}

#[no_mangle]
/// Destructor
pub extern "C" fn biquad8_delete(biquad8: *mut Biquad8Opaque) {
  if !biquad8.is_null() {
    unsafe {drop(Box::from_raw(biquad8 as *mut Biquad8))}
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_process(biquad8: *mut Biquad8Opaque, sample: f32) -> f32 {
  (*(biquad8 as *mut Biquad8)).process(sample)
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_update(biquad8: *mut Biquad8Opaque, coeffs: BiquadCoeffs) {
  (*(biquad8 as *mut Biquad8)).update(coeffs);
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_calc_lpf(biquad8: *mut Biquad8Opaque, w: f32, q: f32) {
  (*(biquad8 as *mut Biquad8)).update(BiquadCoeffs::lpf(w, q));
}
    
#[no_mangle]
pub unsafe extern "C" fn biquad8_calc_bpf(biquad8: *mut Biquad8Opaque, w: f32, q: f32) {
  (*(biquad8 as *mut Biquad8)).update(BiquadCoeffs::bpf(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_calc_hpf(biquad8: *mut Biquad8Opaque, w: f32, q: f32) {
  (*(biquad8 as *mut Biquad8)).update(BiquadCoeffs::hpf(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_calc_notch(biquad8: *mut Biquad8Opaque, w: f32, q: f32) {
  (*(biquad8 as *mut Biquad8)).update(BiquadCoeffs::notch(w, q));
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_calc_peq(biquad8: *mut Biquad8Opaque, w: f32, q: f32, gain: f32) {
  (*(biquad8 as *mut Biquad8)).update(BiquadCoeffs::peq(w, q, gain));
}
// CALCULATE COEFFS:

#[no_mangle]
pub extern "C" fn calc_lpf(w: f32, q: f32) -> BiquadCoeffs {
  crate::filter::biquad::calc::lpf(w, q)
}
    
#[no_mangle]
pub extern "C" fn calc_bpf(w: f32, q: f32) -> BiquadCoeffs {
  crate::filter::biquad::calc::bpf(w, q)
}

#[no_mangle]
pub extern "C" fn calc_hpf(w: f32, q: f32) -> BiquadCoeffs {
  crate::filter::biquad::calc::hpf(w, q)
}

#[no_mangle]
pub extern "C" fn calc_notch(w: f32, q: f32) -> BiquadCoeffs {
  crate::filter::biquad::calc::notch(w, q)
}

#[no_mangle]
pub extern "C" fn calc_peq(w: f32, q: f32, gain: f32) -> BiquadCoeffs {
  crate::filter::biquad::calc::peq(w, q, gain)
}
