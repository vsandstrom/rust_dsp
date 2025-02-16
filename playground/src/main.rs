use std::{
  sync::mpsc::channel,
  thread,
  time::Duration
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use rust_dsp::delay::DelayLine;

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
    let mut d: DelayLine<{1<<17}> = match DelayLine::new((1.0 * sr) as usize) {
      Ok(dline) => dline,
      Err(e) => panic!("{}", e)
    };

    // Create a channel to send and receive samples
    let (tx, rx) = channel::<Vec<f32>>();
    // Callbacks
    let input_callback = move 
      | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
      tx.send(data.to_vec());
    };


    let output_callback = move 
      | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
      // Process output data
      if let Ok(input) = rx.recv() {
        for (out_frame, in_frame) in data.chunks_mut(2).zip(input.chunks(2)) {
          let out = d.read_and_write(in_frame[0]);
          out_frame.iter_mut().for_each(
            |sample| {
            *sample = out
            })
        };

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
      thread::sleep(Duration::from_secs(40));
    }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
