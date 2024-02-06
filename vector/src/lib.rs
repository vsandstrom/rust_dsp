use core::marker::PhantomData;
use wavetable::WaveTable;
use interpolation::interpolation::Interpolation;

pub struct VectorOscillator<'a, T> {
  tables: &'a mut Vec<WaveTable<T>>,
  _interpolation: PhantomData<T>,
}

impl<'a, T> VectorOscillator<'a, T> 
  where T: Interpolation {

  /// Create a 1D Vector Oscillator
  pub fn new(tables: &'a mut Vec<WaveTable<T>>) -> Self {
    VectorOscillator { tables, _interpolation: PhantomData }
  }

  /// Position is a value between 0.0 -> 1.0, scrolls through wavetables
  /// Frequency and phase are passed to each of the wavetable objects.
  pub fn play(&mut self, position: f32, frequency: f32, phase: f32) -> f32 {
    let position = if position > 1.0 { 1.0 } else { position };
    let vec_pos = position * (self.tables.len() - 1) as f32;
    let table_1 = vec_pos.floor() as usize;
    let table_2 = vec_pos.ceil() as usize;
    T::interpolate(
      position,
      &[
        self.tables[table_1].play(frequency, phase),
        self.tables[table_2].play(frequency, phase)
      ].to_vec(),
      2
    )
  }
}

#[cfg(test)]
mod tests {
}
