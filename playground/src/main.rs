use std::{
  sync::mpsc::channel,
  thread,
  time::Duration
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use rust_dsp::{adsr::{ADSREnvelope, Reset}, interpolation, waveshape::sine, wavetable::shared::WaveTable, dsp::signal::map};

// type Frame = [f32; 2];


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
    let mut env = ADSREnvelope::new(sr);
    let mut w = vec![0.0; 512];
    sine(&mut w);

    let mut lfo0 = WaveTable::new();
    let mut lfo1 = WaveTable::new();
    let mut lfo2 = WaveTable::new();

    lfo0.set_samplerate(sr);
    lfo1.set_samplerate(sr);
    lfo2.set_samplerate(sr);

    let mut t = true;
    let mut sustain = true;
    let mut c = 0;

    // Create a channel to send and receive samples
    let (tx, _rx) = channel::<f32>();
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
        let out = env.play(t, sustain);
        t = false;
        env.set_attack_dur(map(&mut lfo0.play::<interpolation::Linear>(&w, 0.1, 0.0), -1.0, 1.0, 0.01, 0.5));
        env.set_decay_dur(map(&mut lfo1.play::<interpolation::Linear>(&w, 0.2, 0.0), -1.0, 1.0, 0.1, 0.8));
        env.set_attack_cur(map(&mut lfo2.play::<interpolation::Linear>(&w, 0.45, 0.0), -1.0, 1.0, 0.5, 1.5));

        if c == (48000.0*1.5) as usize { sustain = false; }
        if c == (48000.0 * 2.0) as usize {
          t = true;
          sustain = true;
          c = 0;
        }
        c+=1;

        

        frame.iter_mut().for_each(
          |sample| {
          *sample = out
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
      thread::sleep(Duration::from_secs(40));
    }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
