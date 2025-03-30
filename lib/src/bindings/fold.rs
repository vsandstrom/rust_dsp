use crate::fold::{Fold, Abs, Sin, Tanh};

#[no_mangle]
pub extern "C" fn fold_abs_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Abs>(input, amount)
}

#[no_mangle]
pub extern "C" fn fold_sin_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Sin>(input, amount)
}

#[no_mangle]
pub extern "C" fn fold_tanh_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Tanh>(input, amount)
}
