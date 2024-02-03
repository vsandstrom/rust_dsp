use std::{thread, time, usize};
use buffer::Buffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dsp::buffer::traits::SignalVector;
use envelope::Envelope;
use grains::{Grain, Granulator};
use trig::{Trigger, Impulse};
use wavetable::WaveTable;
use std::sync::mpsc::{channel, Receiver, Sender};
use interpolation::interpolation::{Floor, Linear, Cubic};
use waveshape::traits::Waveshape;


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
    let buf_l = Buffer::new(4 * config.sample_rate.0 as usize, f_sample_rate);
    let buf_r = Buffer::new(4 * config.sample_rate.0 as usize, f_sample_rate);
    let env = Envelope::from(vec![0.0; 512].hanning());

    let t: Vec<f32> = vec![0.0; 512].sawtooth().iter().map(|x| (x+1.0) / 2.0).collect();
    let mut phl = WaveTable::<Floor>::new(&t.clone(), f_sample_rate);
    let mut phr = WaveTable::<Floor>::new(&t, f_sample_rate);

    let mut tl = Impulse::new(0.15, f_sample_rate);
    let mut tr = Impulse::new(0.19, f_sample_rate);

    let mut gl = Granulator::<Cubic, Linear, Cubic>::new(buf_l, env.clone(), f_sample_rate, 16);
    let mut gr = Granulator::<Cubic, Linear, Cubic>::new(buf_r, env, f_sample_rate, 16);

    gl.set_jitter(0.3);
    gr.set_jitter(0.3);

    let time_at_start = std::time::Instant::now();
    
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
        let t = time_at_start.elapsed().as_secs();
        let mut input_fell_behind = false;
        for sample in data {
          // Recieve sample from input stream
          let raw_sample = rx.recv();
          *sample = match raw_sample { 
            Ok(sample) =>{
              if t < 5 {
                if ch % 2 == 0 {
                  ch+=1;
                  gl.record(sample);
                } else {
                  ch+=1;
                  gr.record(sample);
                }
                sample
              } else {
                let pl = phl.play(1.0/10.0, 0.0);
                let pr = phr.play(1.0/6.0, 0.0);
                if ch % 2 == 0 {
                  ch += 1;
                  gl.play(0.5, 5.0/4.0, pl, tl.play(0.45))
                } else {
                  ch += 1;
                  gr.play(1.4, 3.0/2.0, pr, tr.play(1.0))
                }
              }
              // Stereo handling in interleaved stream
              // sample
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
