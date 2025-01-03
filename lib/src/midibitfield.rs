#[cfg(not(feature="std"))]
use alloc::{format, string::String};


#[derive(Default)]
pub struct MidiBitField { data: u128 /* default: 0 */ }

impl MidiBitField {
  /// Creates a new instance of `MidiBitField`, with its underlying structure 
  /// initialized to `0`.
  pub fn new() -> Self { Self::default() }

  /// Stores Note on-data
  // Encodes a u8 value to a bit in an u128 integer.
  // note: `0` sets the 0th bit, making the binary representation `0b001`
  // `0` sets the 1st bit, making the binary representation `0b010` etc.
  pub fn add(&mut self, value: u8) -> Result<(), &'static str> {
    if value == 128 { return Err("128 is not a valid MIDI note input") }
    self.data |= 1<<value;
    Ok(())
  }

  /// Resets note on-data, effectively `note off`
  // Resets a bit if it is set high.
  pub fn remove(&mut self, value: u8) -> Result<(), &'static str> {
    if value == 128 { return Err("128 is not a valid MIDI note input") }
    self.data &= !(1<<value);
    Ok(())
  }

  /// Will call a function for every active note
  pub fn notes(&self, func: &mut impl FnMut(u8)) {
    if self.data == 0 { return }
    let mut data = self.data;

    while data != 0 {
      let note = data.trailing_zeros();
      func(note as u8);
      data &= !(1<<note);
    }
  }

  #[allow(unused)]
  /// returns the value of the underlying structure
  pub(crate) fn get_data(&self) -> u128 {
    self.data
  }

  /// Frees all voices - MIDI note `Panic`
  pub fn reset(&mut self) {
    self.data = 0;
  }

  /// Prints the binary encoding of the underlying structure, a u128
  pub fn repr(&self) -> String {
    format!("{:b}", self.data)
  }

}

#[cfg(test)]
mod tests {
use super::*;
  #[test]
  /// activate bit 4, (16)
  fn add_bit_4() {
    let mut bm = MidiBitField::default();
    bm.add(4).unwrap();
    assert_eq!(bm.get_data(), 16)
  }
  
  #[test]
  /// activate bit 4 and bit 0, (16 + 1)
  fn add_bit_4_and_0() {
    let mut bm = MidiBitField::default();
    bm.add(4).unwrap();
    bm.add(0).unwrap();
    assert_eq!(bm.get_data(), 17)
  }

  #[test]
  /// Handles deactivating bit correctly
  fn removal() {
    let mut bm = MidiBitField::default();
    for i in 0..128 {
      bm.add(i).unwrap();
    }

    bm.remove(0).unwrap();

    assert_eq!(bm.get_data(), u128::MAX - 1);
  }

  #[test]
  /// When all active bits have been deactivated
  fn zeroed_out() {
    let mut bm = MidiBitField::default();
    bm.add(8).unwrap();
    bm.remove(8).unwrap();
    assert_eq!(bm.get_data(), 0);
  }

  #[test]
  /// checks if bitfield handles the one value not allowed correctly.
  fn error_128() {
    let mut bm = MidiBitField::default();
    let x = bm.add(128);
    assert!(x.is_err())
  }

  #[test]
  fn representation() {
    let mut bm = MidiBitField::default();
    bm.add(2).unwrap();

    assert_eq!("100", bm.repr());
  }

}
