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
  polytable::{PolyTable, PolyVector},
  trig::{Dust, Impulse, TrigTrait},
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
    let sr = config.sample_rate.0 as f32;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    const SIZE: usize = 1 << 12;


    let mut triggers = [false; 9];
    let trig = Impulse::new(sr);

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
      BreakPoint{value: 0.8, duration: 2.2, curve: None},
      BreakPoint{value: 0.4, duration: 0.8, curve: None},
      BreakPoint{value: 0.3, duration: 1.2, curve: None},
      BreakPoint{value: 0.0, duration: 1.2, curve: None},
    ], sr).unwrap();

    let mut position_env = Envelope::new([
      BreakPoint{value: 0.0, duration: 0.1, curve: None}, 
      BreakPoint{value: 0.2, duration: 0.1, curve: None}, 
      BreakPoint{value: 0.8, duration: 0.1, curve: None}, 
      BreakPoint{value: 0.1, duration: 0.1, curve: None}, 
    ], sr).unwrap();
    position_env.set_loopable(true);

    let mut envs = [env; 5];


    let mut wv = WaveTable::new();
    wv.set_samplerate(sr);

    let mut pv: PolyVector<5> = PolyVector::new(sr);
    // pv.set_samplerate(sr);

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
      let inner_time = Instant::now().duration_since(time).as_secs_f32();
      if inner_time > 2.5 && !triggers[0] {
        triggers[0]=true;
        pv.trigger(Some(300.0));
        envs[0].trig();
      } else if inner_time > 3.7 && !triggers[1] {
        triggers[1] = true;
        pv.trigger(Some(225.0));
        envs[1].trig();
      } else if inner_time > 4.0 && !triggers[2] {
        triggers[2] = true;
        pv.trigger(Some(450.0));
        envs[2].trig();
      } else if inner_time > 5.2 && !triggers[3] {
        triggers[3] = true;
        pv.trigger(Some(800.0));
        envs[3].trig();
      } else if inner_time > 6.3 && !triggers[4] {
        triggers[4] = true;
        pv.trigger(Some(350.0));
        envs[4].trig();
      } else if inner_time > 7.4 && !triggers[5] {
        triggers[5] = true;
        pv.trigger(Some(500.0));
        envs[0].trig();
      } else if inner_time > 8.5 && !triggers[6] {
        triggers[6] = true;
        pv.trigger(Some(377.0));
        envs[1].trig();
      } else if inner_time > 9.6 && !triggers[7] {
        triggers[7] = true;
        envs[2].trig();
        pv.trigger(Some(275.0));
        envs[3].trig();
      } else if inner_time > 10.7 && !triggers[8] {
        triggers[8] = true;
        pv.trigger(Some(900.0/2.9));
        envs[4].trig();
      } 
      // Process output data
      for frame in data.chunks_mut(2) {
        frame.iter_mut().for_each(
          |sample| {
          let pos = position_env.play();
          *sample = 
            pv.play::<SIZE, Linear>(
              &tables,
              &[pos; 5],
              &[0.0; 5], 
              &mut |sig, i| {sig * envs[i].play()})
          })
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


    loop{
      thread::sleep(time::Duration::from_secs(40));
    }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
