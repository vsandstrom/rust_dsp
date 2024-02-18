use std::{thread, time, usize};
use buffer::Buffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use delay::{ Delay, IDelay, DelayTrait };
use std::sync::mpsc::{channel, Receiver, Sender};
use interpolation::interpolation::{Floor, Linear, Cubic};


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
    // let latency_frames = (150.0 / 1000.0) * f_sample_rate;
    // let latency_samples = latency_frames as usize * config.channels as usize;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    // let mut dll = IDelay::<Linear>::new(1.6, 1.6, 20, f_sample_rate);
    // let mut dlr = IDelay::<Linear>::new(1.6, 1.6, 20, f_sample_rate);
    let dtl = [0.034, 0.068, 0.136, 0.272];
    let mut ldll = dtl.map(|dt| Delay::new(dt, dt, 1, f_sample_rate));
    let mut ldlr = dtl.map(|dt| Delay::new(dt, dt, 1, f_sample_rate));
    
    let dtl = [8.3, 15.1, 37.953, 77.98, 24.9, 33.21, 55.4, 127.45];
    let mut dll = dtl.map(|dt| IDelay::<Linear>::new(dt / f_sample_rate, dt / f_sample_rate, 1, f_sample_rate));
    let mut dlr = dtl.map(|dt| IDelay::<Linear>::new(dt / f_sample_rate, dt / f_sample_rate, 1, f_sample_rate));

    let mx1 = [1.0, -1.0, 1.0, -1.0];
    let mx2 = [
      1.0, -1.0, 1.0, -1.0,
      1.0, -1.0, 1.0, -1.0
    ];
    // let mxr = [1.0, -1.0, -1.0, 1.0];

    let mut prl = 0.0;
    let mut prr = 0.0;
    
    // let mut dll = Delay::new(1.6, 1.6, 1, f_sample_rate);
    // let mut dlr = Delay::new(1.6, 1.6, 1, f_sample_rate);

    // let time_at_start = std::time::Instant::now();
    
    // Create a channel to send and receive samples
    let (tx, rx): (Sender<f32>, Receiver<f32>) = channel();

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
        let mut ch = 0;
        let mut input_fell_behind = false;
        for sample in data {
          // Recieve sample from input stream
          let raw_sample = rx.recv();
          *sample = match raw_sample { 
            Ok(sample) =>{
              // hacky handler of interleaved stereo
              if ch % 2 == 0 {
                ch+=1;
                let mut out = 0.0;
                for i in 0..8 { out += dll[i].play(sample, 0.44) * mx2[i]; }
                for i in 0..4 { out += (ldll[i].play(out, 0.0) * mx1[i]) / (i as f32 + 1.0); }
                // lowpass
                // let temp = prl + out;
                // prl = out;
                // temp * 0.1
                out * 0.1
              } else {
                ch+=1;
                let mut out = 0.0;
                for i in 0..8 { out += dlr[i].play(sample, 0.44) * mx2[i]; }
                for i in 0..4 { out += (ldlr[i].play(out, 0.0) * mx1[i]) / (i as f32 + 1.0); }
                // lowpass
                // let temp = prr + out;
                // prr = out;
                // temp * 0.1
                out * 0.1
              }
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
