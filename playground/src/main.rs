use std::{f32::consts::PI, thread, time, usize};
use buffer::Buffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use delay::{ Delay, IDelay, DelayTrait };
// use dsp::math::next_pow2;
use grains2::Granulator2;
use trig::{Impulse, Trigger};
use wavetable::WaveTable;
use envelope::{Envelope, BreakPoints};
use std::sync::mpsc::{channel, Receiver, Sender};
use interpolation::interpolation::{Linear, Cubic, Floor};
use waveshape::traits::Waveshape;
// use filter::biquad::{Biquad, calc_bpf};


fn main() -> anyhow::Result<()> {
    // List all audio devices
    let host = cpal::default_host();
    // let devices = host.devices().expect("Devices not found");
    // for device in devices {
    //     println!("{}", device.name().expect("No name?"));
    // }

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

    // Calculate size of ringbuffer
    // let latency_frames = (150.0 / 1000.0) * f_sample_rate;
    // let latency_samples = latency_frames as usize * config.channels as usize;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    const SIZE: usize = 512;

    let bkr = BreakPoints::<3, 2>{
      values: [0.0, 1.0, 0.0], 
      durations: [0.2, 1.45], 
      curves: None
    };

    let mut pos = 0;
    let mut phasor = [0.0f32; SIZE];
    let table = phasor.phasor();
    let mut ph = WaveTable::new(&table, f_sample_rate);
    let mut imp = Impulse::new(0.3, f_sample_rate);
    let buf = Buffer::<{8*48000}>::new(f_sample_rate);

    let mut gr: Granulator2<8, {8*48000}> = Granulator2::new(
      // Buffer::new(f_sample_rate), 
      // Envelope::new(&bkr, f_sample_rate),
      f_sample_rate,
    );
    
    // Create a channel to send and receive samples
    let (tx, rx) = channel::<f32>();

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
        let mut right = 0.0;
        for sample in data {
          // Recieve sample from input stream
          if let Ok(_sample) = rx.recv() {
            if let Some(rec) = gr.record(_sample) {
              *sample = rec;
            } else {
              *sample = 0.0
            }
            *sample = 0.0
          } else {
            *sample = 0.0;
          }

          // *sample = match raw_sample { 
            // Ok(sample) =>{
            //   // hacky handler of interleaved stereo
            //   if ch  == 0 {
            //     // if let Some(sample) = gr.record(sample) { 
            //     //   right = sample;
            //     // } else { 
            //       right = gr.play::<Linear, Cubic>(
            //         1.0, 
            //         0.2, 
            //         ph.play::<Linear>(0.2, 0.0), 
            //         imp.play(0.2)
            //       ); 
            //     // }
            //   }
            //   ch = (ch + 1) % 2;
            //   right
            // },
            // Err(_) => {
            //   input_fell_behind = true;
            //   0.0
            // }
          // }
        }

        if input_fell_behind { eprintln!("Input fell behind"); }
    };
    

    let err_callback = |err: cpal::StreamError| {
        eprintln!("{}", err);
    };

    if let Ok(input_stream) = input_device.build_input_stream(
        &config, 
        input_callback,
        err_callback,
        None
    ) {
      input_stream.play().expect("FAILED INPUT STREAM");
    }

    if let Ok(output_stream) = output_device.build_output_stream(
        &config,
        output_callback,
        err_callback,
        None
    ) {
      output_stream.play().expect("FAILED OUTPUT STREAM");
    }


    thread::sleep(time::Duration::from_secs(40));

    Ok(())
}
