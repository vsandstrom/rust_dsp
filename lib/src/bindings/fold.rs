use crate::fold::{Abs, Fold, Sin, Tanh, FoldType};

#[unsafe(no_mangle)]
pub extern "C" fn fold_abs_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Abs>(input, amount)
}

#[unsafe(no_mangle)]
pub extern "C" fn fold_sin_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Sin>(input, amount)
}

#[unsafe(no_mangle)]
pub extern "C" fn fold_tanh_process(input: f32, amount:f32) -> f32 {
  Fold::process::<Tanh>(input, amount)
}

#[unsafe(no_mangle)]
pub extern "C" fn fold_process(input: f32, amount:f32, foldtype: FoldType) -> f32 {
  Fold::process_dyn(input, amount, foldtype)
}
