use std::{
  sync::mpsc::channel,
  thread, 
  time::{self, Instant}, 
  ops::Add,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use rand::Rng;
use rust_dsp::{
  delay::{Delay, DelayTrait, FixedDelay}, 
  dsp::buffer::traits::SignalVector,
  envelope::new_env::{BreakPoint, Envelope},
  grains::{stereo::Granulator, GrainTrait},
  interpolation::{self, Cubic, Linear},
  polytable::PolyVector,
  trig::{Dust, Impulse, Trigger},
  waveshape::traits::Waveshape,
  wavetable::shared::WaveTable 
};

type Frame = [f32; 2];

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
    // println!("{:#?}", config);

    let f_sample_rate = config.sample_rate.0 as f32;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    const SIZE: usize = 1 << 12;


    let trig = Impulse::new(f_sample_rate);

    let t1 = [0.0; SIZE].complex_sine(
      [1.0, 0.2, 0.5, 0.8],
      [0.0, 0.1, 0.8, 1.2]
    );
    
    let t2 = [0.0; SIZE].sine();
    let t3 = [0.0; SIZE].triangle();
    let t4 = [0.0; SIZE].sawtooth();

    let tables = [t1, t2, t3, t4];

    let mut env = Envelope::new([
      BreakPoint{value: 0.0, duration: 0.2, curve: None},
      BreakPoint{value: 1.0, duration: 0.2, curve: None},
      BreakPoint{value: 0.0, duration: 0.2, curve: None},
      BreakPoint{value: 0.4, duration: 0.8, curve: None},
      BreakPoint{value: 0.0, duration: 1.2, curve: None},
      BreakPoint{value: 0.4, duration: 0.8, curve: None},
      BreakPoint{value: 0.0, duration: 1.2, curve: None},
      BreakPoint{value: 0.4, duration: 0.8, curve: None},
      BreakPoint{value: 0.0, duration: 1.2, curve: None},
    ], f_sample_rate).unwrap();

    env.set_loopable(true);
    env.trigger();

    let mut wv = WaveTable::new();
    wv.set_samplerate(f_sample_rate);

    // Create a channel to send and receive samples
    let (tx, _rx) = channel::<f32>();
    let time = Instant::now();

    // Callbacks
    let input_callback = move 
      | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
      let mut output_fell_behind = false;
      for &sample in data {
          // Send input data to the output callback, or do any processing
        if tx.send(sample).is_err() {
          output_fell_behind = true;
        }
      }
      if output_fell_behind { eprintln!("Output fell behind"); }
    };

    let output_callback = move 
      | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
      // Process output data
      for frame in data.chunks_mut(2) {
        frame.iter_mut().for_each(
          |sample| 
          *sample = (
            wv.play::<SIZE, Linear>(&tables[3], 200.0, 0.0) ) * env.play()
          );

        // SORRY FOR THE STUPID POLY HANDLING!!!!
        // polyvector is built specifically for
        // triggering on midi note on
      }
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


    loop{
      thread::sleep(time::Duration::from_secs(40));
    }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
