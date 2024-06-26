use std::{
  sync::{
    mpsc::channel, Arc, RwLock
  }, thread, time::{self, Instant}, usize
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dsp::buffer::traits::SignalVector;
use grains::Granulator;
use trig::{Dust, Trigger};
use wavetable::owned::WaveTable;
use interpolation::interpolation::{Linear, Cubic};
use waveshape::{hanning, traits::Waveshape};
use envelope::{BreakPoints, EnvType::{self}};
use polytable::vector::PolyVector;
use reverb::SchroederVerb;


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

    // let brk = BreakPoints::<3, 2>{
    //   values: [0.0, 1.0, 0.0], 
    //   durations: [0.2, 1.45], 
    //   curves: Some([0.2, 1.8])
    // };

    let tables = Arc::new(RwLock::new([
      [0.0; SIZE].complex_sine([1.0, 0.2, 0.5, 0.8], [0.0, 0.1, 0.8, 1.2]).to_owned(),
      [0.0; SIZE].sine().to_owned(),
      [0.0; SIZE].triangle().to_owned(),
    ].to_vec()));

    let mut poly: PolyVector<8, SIZE> = PolyVector::new(tables.clone(), f_sample_rate);

    let shape = EnvType::BreakPoint(
      BreakPoints { values: [0.0, 1.0, 0.3, 0.0], durations: [0.2, 2.2, 4.0], curves: None }
    );
    poly.update_envelope(&shape);
    let mut lfo = WaveTable::new(&[0.0; SIZE].triangle().scale(0.0, 1.0), f_sample_rate);
    let mut dlfo = WaveTable::new(&[0.0; SIZE].triangle().scale(0.0, 1.0), f_sample_rate);
    let mut rlfo = WaveTable::new(&[0.0; SIZE].triangle(), f_sample_rate);
    let mut tlfo = WaveTable::new(&[0.0; SIZE].triangle().scale(0.0, 1.0), f_sample_rate);
    

    let gr_env: envelope::EnvType = envelope::EnvType::Vector(hanning(&mut [0.0; 1024]).to_owned());
    // let gr_env: envelope::EnvType<3, 2> = EnvType::BreakPoint(BreakPoints { values: [0.0, 1.0, 0.0], durations: [0.5, 1.2], curves: Some([0.7, 1.2]) });
    let mut gr: Granulator<16, {48000*5}> = Granulator::new(gr_env, f_sample_rate);
    let mut trig = Dust::new(f_sample_rate);
    let mut phasor = WaveTable::new([0.0;SIZE].phasor(), f_sample_rate);

    // Create a channel to send and receive samples
    let (tx, _rx) = channel::<f32>();
    let time = Instant::now();

    let mut triggers = [false; 9];

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
      let mut note = None;
      let mut out = 0.0;
      let inner_time = Instant::now().duration_since(time).as_secs_f32();
      for sample in data {
        // SORRY FOR THE STUPID POLY HANDLING!!!!
        // polyvector is built specifically for
        // triggering on midi note on
        if inner_time > 1.5 && !triggers[0] {
          triggers[0] = true;
          note = Some(300.0);
        } else if inner_time > 1.7 && !triggers[1] {
          triggers[1] = true;
          note = Some(225.0);
        } else if inner_time > 2.0 && !triggers[2] {
          triggers[2] = true;
          note = Some(450.0);
        } else if inner_time > 2.2 && !triggers[3] {
          triggers[3] = true;
          note = Some(800.0);
        } else if inner_time > 2.3 && !triggers[4] {
          triggers[4] = true;
          note = Some(350.0);
        } else if inner_time > 2.4 && !triggers[5] {
          triggers[5] = true;
          note = Some(500.0);
        } else if inner_time > 2.5 && !triggers[6] {
          triggers[6] = true;
          note = Some(377.0);
        } else if inner_time > 2.6 && !triggers[7] {
          triggers[7] = true;
          note = Some(275.0);
        } else if inner_time > 2.7 && !triggers[8] {
          triggers[8] = true;
          note = Some(900.0/2.9);
        } 

        if ch == 0 {
          out = {
            let mut out = poly.play::<Linear, Linear>(
              note,
              &[lfo.play::<Linear>(0.15, 0.0); 8],
              // &[0.5; 8],
              &[0.0; 8]
            ) * 0.1;
            note = None;
            if let Some(sample) = gr.record(out) {
              out = sample;
            } else {
              out += gr.play::<Linear, Linear>(
                phasor.play::<Linear>(1.0/8.0, 0.0),
                0.35 + dlfo.play::<Linear>(4.38, 0.0) * 0.4,
                1.0 + rlfo.play::<Linear>(1.8, 0.0) * 0.04,
                0.0001,
                trig.play(0.02 + tlfo.play::<Linear>(0.2, 0.0) * 0.1) 
              ) * 0.1;
            }
            out
            }
          }
        ch = (ch + 1) % 2;
        *sample = out;
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

    #[allow(unreachable_code)]
    Ok(())
}
