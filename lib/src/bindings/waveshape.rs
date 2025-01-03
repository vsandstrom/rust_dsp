use crate::waveshape;
use alloc::slice;

#[no_mangle]
pub unsafe extern "C" fn shape_complex_sine(table: *mut f32, size: usize, amps: *const f32, asize: usize, phases: *const f32, psize: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  let amps = slice::from_raw_parts(amps, asize);
  let phases = slice::from_raw_parts(phases, psize);
  waveshape::complex_sine(table, amps, phases);
}

#[no_mangle]
pub unsafe extern "C" fn shape_sine(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::sine(table);
}


#[no_mangle]
pub unsafe extern "C" fn shape_hanning(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::hanning(table);
}

#[no_mangle]
pub unsafe extern "C" fn shape_square(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::square(table);
}

#[no_mangle]
pub unsafe extern "C" fn shape_triangle(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::triangle(table);
}

#[no_mangle]
pub unsafe extern "C" fn shape_reverse_sawtooth(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::reverse_sawtooth(table);
}

#[no_mangle]
pub unsafe extern "C" fn shape_sawtooth(table: *mut f32, size: usize) {
  let table = slice::from_raw_parts_mut(table, size);
  waveshape::sawtooth(table);
}
