use std::{thread, time, usize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{channel, Receiver, Sender};


fn main() -> anyhow::Result<()> {
    // List all audio devices
    let host = cpal::default_host();
    let devices = host.devices().expect("Devices not found");
    for device in devices {
        println!("{}", device.name().expect("No name?"));
    }

    // List default input and output devices
    let input_device = match host.default_input_device() {
      Some(device) => {println!("Default input: {}", device.name().unwrap()); device},
      None => panic!("no default input device available")
    };

    let output_device = match host.default_output_device() {
      Some(device) => {println!("Default output: {}", device.name().unwrap()); device},
      None => panic!("no default output device available")
    };

    // Use default config from input device
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();
    println!("{:#?}", config);

    let f_sample_rate = config.sample_rate.0 as f32;
    // Calculate size of ringbuffer
    let latency_frames = (150.0 / 1000.0) * f_sample_rate;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // Create a ringbuffer to store samples
    let (tx, rx): (Sender<f32>, Receiver<f32>) = channel();

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    //
    //
    //
    //
    //
    //
    //

    let time_at_start = std::time::Instant::now();
    
    // Callbacks
    let input_callback = move | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
        let mut output_fell_behind = false;
        for &sample in data {
          // Send input data to the output callback, or do any processing
          match tx.send(sample) {
            Err(_) => output_fell_behind = true,
            _ => ()
          }
        }

        if output_fell_behind { eprintln!("Output fell behind"); }
    };

    let output_callback = move | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
        // Process output data
        let mut input_fell_behind = false;
        for sample in data {
          // Recieve sample from input stream
          let raw_sample = rx.recv();
          *sample = match raw_sample { 
            Ok(sample) =>{
              // DO AUDIO PROCESSING HERE <------------------------------
              0.0
            },
            Err(_) => {
              input_fell_behind = true;
              0.0
            }
          }
        }

        if input_fell_behind { eprintln!("Input fell behind"); }
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

    input_stream.play()?;
    output_stream.play()?;

    thread::sleep(time::Duration::from_secs(40));

    Ok(())
}
