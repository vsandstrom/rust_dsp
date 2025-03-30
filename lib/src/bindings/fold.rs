use crate::fold::Fold;

#[no_mangle]
pub extern "C" fn fold_process(input: f32, amount:f32) -> f32 {
  Fold::process(input, amount)
}


