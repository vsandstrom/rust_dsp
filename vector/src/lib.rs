use wavetable::single::WaveTable;
use interpolation::interpolation::InterpolationConst;

pub struct VectorOscillator<'a, const N: usize> {
  tables: &'a mut [WaveTable<N>],
}

impl<'a, const N:usize> VectorOscillator<'a, N> {

  /// Create a 1D Vector Oscillator
  pub fn new(tables: &'a mut [WaveTable<N>]) -> Self {
    VectorOscillator { tables }
  }

  /// Position is a value between 0.0 -> 1.0, scrolls through wavetables
  /// Frequency and phase are passed to each of the wavetable objects.
  pub fn play<T: InterpolationConst>(&mut self, position: f32, frequency: f32, phase: f32) -> f32 {
    let position = if position > 1.0 { 1.0 } else { position };
    let vec_pos = position * (self.tables.len() - 1) as f32;
    let table_1 = vec_pos.floor() as usize;
    let table_2 = vec_pos.ceil() as usize;
    T::interpolate(
      position,
      &[
        self.tables[table_1].play::<T>(frequency, phase),
        self.tables[table_2].play::<T>(frequency, phase)
      ],
      2
    )
  }
}

#[cfg(test)]
mod tests {
}
