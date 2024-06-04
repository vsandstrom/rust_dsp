use std::{f32::consts::PI, thread, time, usize};
use buffer::Buffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use delay::{ Delay, IDelay, DelayTrait };
use dsp::math::next_pow2;
use grains::Granulator;
use trig::{Impulse, Trigger};
use wavetable::WaveTable;
use envelope::{Envelope, BreakPoints};
use std::sync::mpsc::{channel, Receiver, Sender};
use interpolation::interpolation::{Linear, Cubic, Floor};
use waveshape::traits::Waveshape;
use filter::biquad;


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
    let bkr = BreakPoints::<3, 2>{
      values: [0.0, 1.0, 0.0], 
      durations: [0.2, 1.45], 
      curves: None
    };

    let mut env2 = Envelope::new(&bkr, f_sample_rate);

    let mut phasor = [0.0f32; 512];
    let table = phasor.phasor();
    let mut ph = WaveTable::new(&table, f_sample_rate);

    let mut imp = Impulse::new(0.3, f_sample_rate);

    let mut gr: Granulator<8, {8*48000}> = Granulator::new(
      Buffer::new(f_sample_rate), 
      Envelope::new(&bkr, f_sample_rate),
      f_sample_rate,
    );
    
  
    const SIZE: usize = 512;
    let mut table = [0.0; SIZE];
    let amps = [1.0, 3.0, 0.4, 0.7, 2.0];
    let phas = [0.0, 0.33*PI, 0.0, 0.0, PI];

    const DSIZE1: usize = next_pow2((0.8 * 48000.0) as usize);
    const DSIZE2: usize = next_pow2((0.707 * 48000.0) as usize);

    
    let mut dlyr = IDelay::<DSIZE1>::new(6, f_sample_rate);
    let mut dlyl = IDelay::<DSIZE2>::new(7, f_sample_rate);

    let mut envtable = [0.0; SIZE];
    let mut env = WaveTable::<SIZE>::new(envtable.hanning(), f_sample_rate);

    let mut wt1 = WaveTable::<SIZE>::new(
      table.complex_sine(amps, phas),
      f_sample_rate
    );
    let mut wt2 = wt1.clone();
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
        let mut input_fell_behind = false;
        let mut right = 0.0;
        for sample in data {
          // Recieve sample from input stream
          let e = env2.play::<Floor>(imp.play(2.2));
          let raw_sample = rx.recv();
          *sample = match raw_sample { 
            Ok(sample) =>{
              // hacky handler of interleaved stereo
              if ch % 2 == 0 {
                ch+=1;
                if let Some(sample) = gr.record(sample) { 
                  right = sample;
                  right
                } else { 
                  right = gr.play::<Linear, Cubic>(
                    1.0, 
                    0.2, 
                    ph.play::<Linear>(0.2, 0.0), 
                    imp.play(0.2)
                  ); 
                  right 
                }
                // wt1.play::<Cubic>(200.0, 0.0) * 0.1 
                //   // * env.play::<Linear>(4.0, 0.0) 
                //   * e
                //   + dlyl.play::<Linear>(sample, 0.1) * 0.1
              } else {
                ch+=1;
                // wt2.play::<Cubic>(500.0, 0.0) * 0.1 
                //   * e
                //   + dlyr.play::<Linear>(sample, 0.1) * 0.1
                right
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
