use std::{thread, time, usize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use grains2::Granulator2;
use trig::{Dust, Trigger};
use wavetable::WaveTable;
use std::sync::mpsc::channel;
use interpolation::interpolation::{Linear, Cubic};
use waveshape::traits::Waveshape;
use envelope::BreakPoints;

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

    let brk = BreakPoints::<3, 2>{
      values: [0.0, 1.0, 0.0], 
      durations: [0.2, 1.45], 
      curves: Some([0.2, 1.8])
    };

    let mut phasor = [0.0f32; SIZE];
    let mut lfo1 = [0.0f32; SIZE];
    let table = phasor.phasor();
    let lfo1_t = lfo1.sine();
    let mut ph = WaveTable::new(&table, f_sample_rate);
    let mut lfo1_ph = WaveTable::new(&lfo1_t, f_sample_rate);
    let mut imp = Dust::new(f_sample_rate);
    let mut out = 0.0;

    let mut gr: Granulator2<16, {8*48000}> = Granulator2::new(
      // Buffer::new(f_sample_rate), 
      // Envelope::new(&bkr, f_sample_rate),
      brk,
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
        for sample in data {
          // Recieve sample from input stream
          *sample = match rx.try_recv() {
            Ok(s) => {
              if ch == 0 {
                if let Some(_) = gr.record(s) {
                  out = 0.0;
                } else {
                    out = gr.play::<Linear, Linear>(
                      ph.play::<Linear>(1.0/8.0, 1.0),
                      0.8,
                      (lfo1_ph.play::<Cubic>(0.05, 1.0) + 1.1) * 0.6,
                      0.51,
                      imp.play(0.1)
                    )
                  }
                }
              ch = (ch + 1) % 2;
              out
            },
            Err(e) => {
              eprintln!("{e}");
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

    input_stream.play().expect("FAILED INPUT STREAM");
    output_stream.play().expect("FAILED OUTPUT STREAM");


    loop{
      thread::sleep(time::Duration::from_secs(40));
    }

    Ok(())
}
