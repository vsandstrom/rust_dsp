mod plotter;
mod io;

use std::{ 
  thread,
  time::Duration,
  env::args
};

use plotter::plot_buffer;
use io::IO;

use cpal::traits::{DeviceTrait, StreamTrait};
use rust_dsp::{
  adsr::ADSREnvelope,
  dsp::math::next_pow2,
  filter::onepole::Onepole,
  noise::pink,
  waveshape::traits::Waveshape,
  wavetable::shared::Wavetable
};


const SIZE: usize = next_pow2(4800);

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let plot = args()
    .nth(1)
    .map_or_else(|| false, |x| {x == "plot"});
  // let io = IO::new_default()?;
  let io = IO::new_current()?;
  let sr = io.samplerate;
  let ch = io.channels;

  // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
  let mut op = Onepole::new(sr);
  // let cutoffs = [200.0, 400.0, 600.0, 300.0];
  op.set_cutoff(400.0);
  // let mut i = 0;
  // let mut j = 0;

  // let mut wt = Wavetable::new();
  let table = [0.0; 1028].sawtooth();
  let mut wt = Wavetable::new();
  let mut sus = true;
  wt.set_samplerate(sr);
  // let mut noise = Noise::new(sr);
  let mut white = WhiteNoise::new(0x1234beef);
  if plot {
    plot_buffer(&table, false);
  }

  let mut pink = pink::Noise::new(1234);
  let mut dly = Dly::<SIZE, _>::new(move |x, fb| op.process(x) * fb);


  let input_callback = move 
    | _data: &[f32], _: &cpal::InputCallbackInfo | {
      // Process input data
      // NOP
  };

  let mut adsr = ADSREnvelope::new(sr);
  // adsr.set_attack_val(0.0);
  // adsr.set_attack_dur(0.2);
  // adsr.set_decay_dur(0.4);
  // adsr.set_sustain_val(0.2);
  // adsr.set_release_dur(0.2);
  adsr.set_reset_type(Reset::Hard);
  let mut trig = false;
  let mut c = 0;



  let output_callback = move 
    | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
    // Process output data

    for (i, out_frame) in data.chunks_mut(ch.into()).enumerate() {
      // let sig = {
      //   pink.play() * 0.03
      // };
      // let sig = adsr.play(trig);
      // c += 1;
      // if c as f32 >= 8.8 * sr as f32 { trig = !trig; c = 0; }
      // out_frame[0] = sig;
      let sig = white.process();
      out_frame.iter_mut().for_each(|x| *x = sig);
    };
  };

  let err_callback = |err: cpal::StreamError| {
      eprintln!("{}", err);
  };

  let input_stream = io.input_device.build_input_stream(
      &io.config, 
      input_callback,
      err_callback,
      None
  )?;

  let output_stream = io.output_device.build_output_stream(
      &io.config,
      output_callback,
      err_callback,
      None
  )?;

  input_stream.play().expect("FAILED INPUT STREAM");
  output_stream.play().expect("FAILED OUTPUT STREAM");
  loop{ thread::sleep(Duration::from_secs(40)); }

  // allow running forever
  #[allow(unreachable_code)]
  Ok(())
}


use rust_dsp::dsp::math::is_pow2;
use rust_dsp::filter::Filter;
use rust_dsp::adsr::Reset;

// #[derive(Default)]
// pub struct Dly { position: usize }
//
// impl Dly {
//   pub fn new() -> Self { Self::default() }
//   pub fn play<const N: usize>(&mut self, input: f32, buffer: &mut [f32; N], delay: f32, feedback: f32) -> f32 {
//     // let len = buffer.len();
//     debug_assert!(is_pow2(N));
//     let mask = N - 1;
//     let w = delay.fract();
//     let pos = (self.position + delay as usize) & mask;
//     let a = buffer[pos & mask];
//     let b = buffer[(pos+1) & mask];
//     let out = a + w * (b - a);
//     buffer[self.position] = input + (out * feedback);
//     self.position = self.position.wrapping_sub(1) & mask;
//     out
//   }
// }

pub struct Dly<const N: usize, F> 
  where F: FnMut(f32, f32) -> f32,
{
  buffer: [f32; N],
  position: usize,
  feedback: F
}

impl<const N: usize, F> Dly<N, F> 
  where F: FnMut(f32, f32) -> f32,
{
  pub fn new(fb_callback: F) -> Self { 
    let _ = Pow2::<N>::MASK;
    Self { buffer: [0.0; N], position: 0, feedback: fb_callback }
  }

  /// assumes `delay` is non-zero and non-negative
  #[inline(always)]
  pub fn play(&mut self, input: f32, delay: f32, feedback: f32) -> f32 
  {
    let mask = Pow2::<N>::MASK;
    let del = delay as usize;
    let w = delay - del as f32;
    // read position is `ahead` in buffer (see explaination for decrementation below)
    let pos = (self.position + del) & mask;
    let a = self.buffer[pos & mask];
    let b = self.buffer[(pos + 1) & mask];
    let out = a + w * (b - a);
    self.buffer[self.position] = input + (self.feedback)(out, feedback);
    // moving backwards in buffer allows for cheap wrapping around the zero and
    // bitmask And to get position within range of buffer. 
    self.position = self.position.wrapping_sub(1) & mask;
    out
  }
}

pub struct Pow2<const N: usize>;

impl <const N: usize> Pow2<N> {
  pub const MASK: usize = {
  assert!(is_pow2(N), "N must be 2^M");
  N-1
};
}


struct WhiteNoise {
  seed: u32,
}

impl WhiteNoise {
  const MASK: u32 = 0b1;
  const N: u32 = 9;
  const M: u32 = 24;

  fn new(seed: u32) -> Self {
    Self { seed }
  }
  
  #[inline(always)]
  fn process(&mut self) -> f32 {
    let fb = (
      (self.seed >> Self::N) ^
      (self.seed >> Self::M) ^
      (self.seed >> 1) ^
      (self.seed)) & Self::MASK;

    self.seed = (self.seed << 1) | fb;
    f32::from_bits(0x40000000 | (self.seed >> 9)) - 3.0
  }

  // fn process(&mut self) -> f32 {
  //   let mut num = self.seed;
  //   let state = (self.seed >> Self::N);
  //   let state2 = (self.seed >> Self::M);
  //
  //   num ^= state ^ state2;
  //   self.seed = (self.seed << 1) ^ (num & Self::MASK);
  //   f32::from_bits(0x40000000 | (self.seed >> 9)) - 3.0
  // }
}

