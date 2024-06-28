extern crate envelope; 
extern crate buffer;
use envelope::Envelope;
use interpolation::interpolation::Interpolation;
use buffer::Buffer;
use rand::Rng;
use array_init::array_init;

pub struct Grain<const N:usize> {
  samplerate: f32,
  buf_position: f32,
  env_position: f32,
  rate: f32,
  duration: f32,
  pub active: bool,
}

pub struct Granulator<const N: usize, const M: usize> {
  buffer: Buffer<M>,
  envelope: Envelope,
  samplerate: f32,
  grains: [Grain<M>; N],
  rec_pos: usize,
  jitter: f32,
}

impl<const N: usize, const M:usize> Granulator<N, M> {
  // Interpolation trait allows Buffer, Envelope and Granulator to use different interpolation
  // methods that fit the method signature. Grain will inherit the Granulators T
  /// Creates a new Granulator, with a Buffer of fixed size and an Envelope for Grain volume, 
  /// N: Size of Buffer in samples
  /// M: Maximum number of Grains
  pub fn new(buffer: Buffer<M>, grain_env: Envelope, samplerate: f32) -> Self {
    let grains: [Grain<M>; N] = array_init(|_|
      Grain { 
        samplerate,
        buf_position: 0.0,
        env_position: 0.0,
        rate: 1.0,
        duration: 0.0533333,
        active: false,
      }
    );

    Granulator {
      buffer, 
      envelope: grain_env, 
      samplerate, 
      grains,
      rec_pos: 0,
      jitter: 0.0,
    }
  }

  /// Internal play method when no trigger has been detected.
  fn idle_play<T: Interpolation, U: Interpolation>(&mut self) -> f32 {
    let mut out = 0.0;
    for i in 0..N {
      if self.grains[i].active {
        out += self.grains[i].play::<T, U>(&self.envelope, &self.buffer);
        // update values in grains. 
        self.grains[i].incr_ptrs();
        if self.grains[i].env_position as usize >= self.envelope.len() {
          self.grains[i].env_position = 0.0;
          self.grains[i].buf_position = 0.0;
          self.grains[i].active = false;
        }
      }
    }
    out
  }

  /// takes a trigger generator ( trigger >= 1.0 ) and a buffer position ( 0.0..=1.0 )
  /// T = Interpolation for Buffer, 
  /// U = Interpolation for Envelope
  pub fn play<BufferInterpolation: Interpolation, EnvelopeInterpolation: Interpolation>(&mut self, rate: f32, duration: f32, position: f32, trigger: f32) -> f32 {
    if trigger < 1.0 {return self.idle_play::<BufferInterpolation, EnvelopeInterpolation>()}
    let mut out: f32 = 0.0;
    let mut triggered = false;
    // find next available grain to play
    for i in 0..N {
      // accumulate all active
      if self.grains[i].active {
        out += self.grains[i].play::<BufferInterpolation, EnvelopeInterpolation>(&self.envelope, &self.buffer);
      }
      // activate new grain and set to active
      if !triggered && !self.grains[i].active {
        let random = rand::thread_rng().gen_range(0.0..=1.0) * self.jitter;
        self.grains[i].buf_position = (f32::fract(position + random)) * self.buffer.size as f32;
        self.grains[i].env_position = 0.0;
        self.grains[i].set_rate(rate);
        self.grains[i].active = true;
        self.grains[i].set_duration(duration, self.envelope.len() as f32);
        out += self.grains[i].play::<BufferInterpolation, EnvelopeInterpolation>(&self.envelope, &self.buffer);
        triggered = true;
      }
      self.grains[i].incr_ptrs();
      if self.grains[i].env_position as usize >= self.envelope.len() {
        self.grains[i].active = false;
      }
    }
    out
  }

  pub fn record(&mut self, sample: f32) -> Option<f32> {
    if self.rec_pos as usize >= self.buffer.size {
      return None;
    }
    self.buffer.write(sample, self.rec_pos);
    self.rec_pos += 1;
    Some(sample)
  }

  pub fn clear(&mut self) {
    for i in 0..self.buffer.size {
      self.buffer.buffer[i] = 0.0;
    }
  }
  
  pub fn set_jitter(&mut self, jitter: f32) {
    self.jitter = jitter;
  }

  pub fn buffer_len(&self) -> f32 {
    self.buffer.size as f32 / self.samplerate
  }
}

impl<const N:usize> Grain<N> {
  pub fn play<BufferInterpolation: Interpolation, EnvelopeInterpolation: Interpolation>(&self, env: &Envelope, buffer: &Buffer<N>) -> f32 {
    let mut out = buffer.read::<BufferInterpolation>(self.buf_position);
    out *= env.read::<EnvelopeInterpolation>(self.env_position);
    out
  }
  
  pub fn set_duration(&mut self, duration: f32, envelope_length: f32) {
    self.duration = envelope_length / ((self.samplerate) * duration);
  }

  pub fn set_rate(&mut self, rate: f32) {
    self.rate = rate;
  }

  pub fn incr_ptrs(&mut self) {
    self.buf_position += self.rate;
    self.env_position += self.duration;
  }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    use envelope::BreakPoints;
    use interpolation::interpolation::{Floor, Linear};

    trait GrainTest {
      fn count_active(&self) -> usize;
    }

    impl<const N: usize, const M: usize> GrainTest for Granulator<N, M> {
      fn count_active(&self) -> usize {
        let mut count = 0;
        for i in 0..self.grains.len() {
          if self.grains[i].active == true {
            count+=1;
          }
        }
        count
      }
    }

    #[test]
    fn active() {
      let brk = BreakPoints::<3, 2>{
            values: [0.0, 1.0, 0.0],
            durations: [0.2, 0.2],
            curves: None
      };
      let mut gr = Granulator::<8, 32>::new(
        Buffer::<32>::new(48000.0),
        Envelope::new( &brk, 48000.0),
        48000.0
      );

      gr.play::<Floor, Linear>(1.0, 0.2 , 0.5, 1.0);
      assert_eq!(1, gr.count_active());
    }

    #[test]
    fn passive() {
      let brk = BreakPoints::<3, 2>{
            values: [0.0, 1.0, 0.0],
            durations: [0.2, 0.2],
            curves: None
      };
      let gr = Granulator::<8, 32>::new(
        Buffer::<32>::new(48000.0),
        Envelope::new( &brk, 48000.0),
        48000.0
      );
      assert_eq!(0, gr.count_active());

    }

    #[test]
    fn three_active() {
      let brk = BreakPoints::<3, 2>{
            values: [0.0, 1.0, 0.0],
            durations: [0.2, 0.2],
            curves: None
      };
      let mut gr = Granulator::<8, 32>::new(
        Buffer::<32>::new(48000.0),
        Envelope::new( &brk, 48000.0),
        48000.0
      );
      for _ in 0..3 {
        gr.play::<Floor, Linear>(1.0, 0.2 , 0.5, 1.0);
      }
      assert_eq!(3, gr.count_active());
    }

    #[test]
    fn all_active() {
      const SIZE: usize = 8;
      let brk = BreakPoints::<3, 2>{
            values: [0.0, 1.0, 0.0],
            durations: [0.2, 0.2],
            curves: None
      };
      let mut gr = Granulator::<SIZE, 32>::new(
        Buffer::<32>::new(48000.0),
        Envelope::new( &brk, 48000.0),
        48000.0
      );
      // If no more grains are available at the time, no more will be active than MAX
      for _ in 0..(SIZE+1) {
        gr.play::<Floor, Linear>(1.0, 0.2 , 0.5, 1.0);
      }
      assert_eq!(8, gr.count_active());
    }

}
