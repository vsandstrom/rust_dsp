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
  wavetable::shared::Wavetable,
  filter::{ onepole::Onepole, }, 
  noise::pink::Noise as PinkNoise,
  noise::white::Noise as WhiteNoise,
  noise::brown::Noise as BrownNoise,
  waveshape::{traits::Waveshape},
};

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
  op.set_cutoff(200.0);
  // let mut i = 0;
  // let mut j = 0;

  // let mut wt = Wavetable::new();
  let table = [0.0; 1028].sawtooth();
  let mut wt = Wavetable::new();
  wt.set_samplerate(sr);
  // let mut noise = Noise::new(sr);
  if plot {
    plot_buffer(&table, false);
  }

  let seed = 12345678;

  let mut pink = PinkNoise::new(seed);
  let mut white = WhiteNoise::new(seed);
  let mut brown = BrownNoise::new(seed, sr);
  let mut noise_buf = [0.0; 48000];

  // Create a channel to send and receive samples
  // let (tx, rx) = channel::<Vec<f32>>();
  // Callbacks
  let input_callback = move 
    | _data: &[f32], _: &cpal::InputCallbackInfo | {
      // Process input data
      // NOP
  };


  let output_callback = move 
    | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
    // Process output data

    for (i, out_frame) in data.chunks_mut(ch.into()).enumerate() {
      // if i == sr * 4 { j += 1; j &= 0b11; i = 0; }
      // i+=1;
      let sig1 = white.play();
      let sig2 = pink.play() * 0.03;
      let sig3 = brown.play();

      // let sig = wt.play::<Linear>(&table, 100.0, 0.0) * 0.5;
      // op.set_cutoff(cutoffs[j]);
      // let filt = op.process(sig);
      out_frame[0] = sig1;
      out_frame[1] = sig2;
      out_frame[2] = sig3;
    };
    noise_buf.iter_mut().for_each(|x| *x = 0.0);
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


