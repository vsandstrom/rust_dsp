#[macro_use] extern crate rust_dsp;

use std::{ 
  sync::mpsc::channel,
  thread,
  time::Duration,
  f32::consts::TAU
};

use cpal::traits::{
  DeviceTrait,
  HostTrait,
  StreamTrait
};
use rust_dsp::{ 
  dsp::signal::map, 
  dsp::buffer::range, filter::{
    biquad::{
      twopole::Biquad, BiquadCoeffs, BiquadTrait
    }, svf::{
      SVFCoeffs, SVFTrait, SVFilter
    }, 
    Filter
  }, 
  fold::{Fold, Abs}, 
  interpolation::{Floor, Linear},
  waveshape::traits::Waveshape,
  wavetable::shared::Wavetable,
  noise::Noise,
};

fn main() -> anyhow::Result<()> {
    // List all audio devices
    let host = cpal::default_host();

    // List default input and output devices
    let input_device = match host.default_input_device() {
      Some(device) => {
        // println!("Default input: {}", device.name().unwrap());
        device
      },
      None => panic!("no default input device available")
    };

    let output_device = match host.default_output_device() {
      Some(device) => {
        // println!("Default output: {}", device.name().unwrap()); 
        device
      },
      None => panic!("no default output device available")
    };

    // Use default config from input device
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();
    let ch = config.channels;
    let sr = config.sample_rate.0 as f32;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    // let mut bq= Biquad::new();
    // let mut svf = SVFilter::new();
    let table_1 = sine![0.0f32; 512];
    let table_2 = triangle![0.0f32; 512];

    let mut wt = Wavetable::new();
    let mut lfo = Wavetable::new();

    wt.set_samplerate(sr);
    lfo.set_samplerate(sr);

    // Create a channel to send and receive samples
    let (tx, rx) = channel::<Vec<f32>>();
    // Callbacks
    let input_callback = move 
      | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
      // tx.send(data.to_vec());
    };


    let output_callback = move 
      | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
      // Process output data
      for out_frame in data.chunks_mut(ch.into()) {
        let sig = wt.play::<Linear>(&table_1, 200.2, 0.0);
        // let sig = Fold::process::<Abs>(sig, 1.0 + (0.5 * lfo.play::<Linear>(&table_2, 0.4, 0.0)));
        out_frame[0] = sig*0.02; 
        out_frame[1] = sig*0.02;
      };
    };

    let err_callback = |err: cpal::StreamError| {
        eprintln!("{}", err);
    };

    let input_stream = input_device.build_input_stream(
        &config, 
        input_callback,
        err_callback,
        None
    )?;

    let output_stream = output_device.build_output_stream(
        &config,
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

fn fold(sig: f32, min: f32, max: f32  ) -> f32 {
  let mut s1 = sig;
  let x = s1 - min;
  if s1 >= max {
    s1 = max + max - s1;
    if s1 >= min {return s1}
  } else if s1 < min {
    s1 = min + min - s1;
    if s1 < max {return s1}
  }
  
  if max == min {return min;}
  // ok do the divide
  let range = max - min;
  let range2 = range + range;
  let mut c = x - range2 * f32::floor(x / range2);
  if c >= range { c = range2 - c }
  c + min
}
