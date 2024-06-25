use interpolation::interpolation::InterpolationConst;
use dsp::signal::clamp;

/// Vector Oscillator using an vector of float-arrays to traverse smoothly 
/// through different textures.
pub struct VectorOscillator<const N: usize> {
  tables: Vec<[f32;N]>,
  table_pos: f32,
  samplerate: f32,
}

impl<const N:usize> VectorOscillator<N> {

  /// Create a 1D Vector Oscillator
  pub fn new(tables: Vec<[f32; N]>, samplerate:f32) -> Self {
    VectorOscillator { 
      tables,
      table_pos: 0.0,
      samplerate

    }
  }

  /// Position is a value between 0.0 -> 1.0, scrolls through wavetables
  /// Frequency and phase are passed to each of the wavetable objects.
  pub fn play<VectorInterpolation, TableInterpolation>(&mut self, position: f32, frequency: f32, phase: f32) -> f32 
  where 
      VectorInterpolation: InterpolationConst,
      TableInterpolation: InterpolationConst
  {
    if frequency > (self.samplerate) { return 0.0; }
    let n_f32 = N as f32;

    let position = if position > 1.0 { 1.0 } else { position };
    let vec_pos = position * (n_f32 - 1.0);
    let table_1 = vec_pos.floor() as usize;
    let table_2 = vec_pos.ceil() as usize;
    let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
    let out = VectorInterpolation::interpolate(
      position,
      &[
        TableInterpolation::interpolate(self.table_pos, &self.tables[table_1], N),
        TableInterpolation::interpolate(self.table_pos, &self.tables[table_2], N),
      ],
      2
    );

    self.table_pos += n_f32 / self.samplerate / (frequency * norm_ph);
    while self.table_pos > n_f32 {
      self.table_pos -= n_f32;
    }

    out
  }
}

#[cfg(test)]
mod tests {
}
