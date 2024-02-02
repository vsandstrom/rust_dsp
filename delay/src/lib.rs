
pub struct Delay {
  buffer: Vec<f32>,
  buffer_size: usize,
  samplerate: f32,
  delay_taps: usize,
  delay_time: f32,
  position: usize,
}

impl Delay {
  /// Create new Delay
  /// max_delay_time >= delay_time * delay_taps,
  /// ex: max_delay_time = 1.0, delay_time = 0.2, delay_taps = 5
  pub fn new(delay_time: f32, max_delay_time: f32, delay_taps: usize, samplerate: f32) -> Self {
    let buffer_size = (max_delay_time * samplerate) as usize;
    let buffer = vec![0.0; buffer_size];

    Delay{
      buffer,
      buffer_size,
      delay_time,
      delay_taps,
      samplerate,
      position: 0,
    }
  }

  pub fn from_samples(buffer_size: usize, delay_taps: usize, samplerate: f32) -> Self {
    let buffer = vec![0.0; buffer_size];

    Delay{
      buffer,
      buffer_size,
      delay_time: buffer_size as f32 / samplerate,
      delay_taps,
      samplerate,
      position: 0,
    }
  }


  pub fn play(&mut self, sample: f32, feedback: f32) -> f32 {
    while self.position >= self.buffer_size {
      self.position -= self.buffer_size
    }
    let out = self.buffer[self.position];
    self.buffer[self.position] = 0.0;
    for i in 1..=self.delay_taps {
      let mut delay = ((self.delay_time * self.samplerate) as usize * i) + self.position;
      while delay >= self.buffer_size {
        delay -= self.buffer_size;
      }
      self.buffer[delay] += (sample + (out * feedback))  * (0.5/i as f32);
    }
    self.position+=1;
    out
  }

  pub fn set_taps(&mut self, delay_taps: usize) {
    self.delay_taps = delay_taps
  }

  pub fn set_time(&mut self, delay_time: f32) {
    if (delay_time * self.samplerate) as usize >= self.buffer_size {
      self.delay_time = self.buffer_size as f32 / self.samplerate;
    }
    self.delay_time = delay_time
  }
}



#[cfg(test)]
mod tests {
}
