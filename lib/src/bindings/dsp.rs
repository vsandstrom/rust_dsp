use crate::dsp;
use crate::alloc::slice;

// SIGNAL NAMESPACE

#[no_mangle]
pub extern "C" fn clamp_signal(signal: f32, bottom: f32, top: f32 ) -> f32 {
  dsp::signal::clamp(signal, bottom, top)
}

  /// Map a signal of range m -> n into new range, x -> y
#[no_mangle]
pub extern "C" fn signal_map(signal: &mut f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
  dsp::signal::map(signal, in_min, in_max, out_min, out_max)
}

#[no_mangle]
pub extern "C" fn signal_dcblock(signal: f32, xm1: f32, ym1: f32 ) -> f32 {
  dsp::signal::dcblock(signal, xm1, ym1)
}
  
  /// Convenience for normalizing a signal to be only positive.
#[no_mangle]
pub extern "C" fn signal_unipolar(mut sample: f32) -> f32 {
  dsp::signal::map(&mut sample, -1.0, 1.0, 0.0, 1.0)
}

#[no_mangle]
/// calculates panning weights for stereo equal power panning.
/// returns a pointer to an array of len 2, [left, right]
pub extern "C" fn signal_pan_exp2(pan: f32) -> *const f32 {
  let x = dsp::signal::pan_exp2(pan);
  [x.0, x.1].as_ptr()
}

#[no_mangle]
/// calculates panning weights for stereo linear panning.
/// returns a pointer to an array of len 2, [left, right]
pub extern "C" fn signal_pan_lin2(pan:f32) -> *const f32 {
  let x = dsp::signal::pan_lin2(pan);
  [x.0, x.1].as_ptr()
}


// BUFFER NAMESPACE

/// Same as map, but for entire buffers. Suitable for normalizing Wavetable buffers.
/// # Safety
/// Reads a raw pointer into a rust slice.
#[no_mangle]
pub unsafe extern "C" fn buffer_range(values: *mut f32, len: usize, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> *const f32 {
  let x = slice::from_raw_parts_mut(values, len);
  dsp::buffer::range(x, in_min, in_max, out_min, out_max).as_ptr()
}


/// Calculates the sum of all values in array
/// # Safety
/// Reads a raw pointer into a rust slice.
#[no_mangle]
pub unsafe extern "C" fn buffer_sum(values: *const f32, len: usize) -> f32 {
  let values = slice::from_raw_parts(values, len);
  crate::dsp::buffer::sum(values)
}
  
/// Normalizes contents of vec, sum of contents == 1.0
/// # Safety
/// Reads a raw pointer into a rust slice.
#[no_mangle]
pub unsafe extern "C" fn buffer_normalize(values: *mut f32, len: usize) {
  let values = slice::from_raw_parts_mut(values, len);
  crate::dsp::buffer::normalize(values)
}

/// Scales the contents of a Vec to be between outmin -> outmax
/// # Safety
/// Reads a raw pointer into a rust slice.
/// (should mutate contents of array in place)
#[no_mangle]
pub unsafe extern "C" fn buffer_scale(values: *mut f32, len: usize, outmin: f32, outmax: f32) {
  let values = slice::from_raw_parts_mut(values, len);
  crate::dsp::buffer::scale(values, outmin, outmax);
}


#[no_mangle]
pub extern "C" fn math_next_pow2(size: usize) -> usize {
  crate::dsp::math::next_pow2(size)
}

#[no_mangle]
pub extern "C" fn math_is_pow2(size: usize) -> bool {
  crate::dsp::math::is_pow2(size)
}

#[no_mangle]
pub extern "C" fn math_midi_to_freq(midi: u8, tuning: f32) -> f32 {
  crate::dsp::math::midi_to_freq(midi, tuning)
}

#[no_mangle]
pub extern "C" fn math_freq_to_midi(freq: f32, tuning: f32) -> u8 {
  crate::dsp::math::freq_to_midi(freq, tuning)
}

#[no_mangle]
pub extern "C" fn math_midi_to_rate(midi: u8) -> f32 {
  crate::dsp::math::midi_to_rate(midi)
}

#[no_mangle]
pub extern "C" fn hz_to_radian(hz: f32, samplerate: f32) -> f32 {
  crate::dsp::math::hz_to_radian(hz, samplerate)
}

// Translate decibel to linear volume
#[no_mangle]
pub extern "C" fn math_db_to_volume(db: f32) -> f32 {
  crate::dsp::math::db_to_volume(db)
}

// Translate  linear volume to decibel
#[no_mangle]
pub extern "C" fn math_volume_to_db(volume: f32) -> f32 {
  crate::dsp::math::volume_to_db(volume)
}

#[no_mangle]
pub extern "C" fn math_samples_to_wavelength(samples: usize, samplerate: f32) -> f32 {
  crate::dsp::math::samples_to_wavelength(samples, samplerate)
  }

#[no_mangle]
pub extern "C" fn wavelength_to_samples(wavelength: f32, samplerate: f32) -> usize {
  crate::dsp::math::wavelength_to_samples(wavelength, samplerate)
}
