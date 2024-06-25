use std::{
  sync::{
    mpsc::channel, Arc, RwLock
  }, thread, time, usize
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use grains::Granulator;
use grains2::Granulator2;
use trig::{Dust, Trigger};
use wavetable::{owned, shared};
// use wavetable2::WaveTable2;
use interpolation::interpolation::{Linear, Cubic};
use waveshape::{sine, traits::Waveshape};
use envelope::{BreakPoints, Envelope};

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

    // let mut pos = 0;
    // let mut started = false;
    // let freq = 100.0;
    // let mut arr = [0.0; 512];
    // let table = arr.sine().to_vec();
    // let xtable = Arc::new(RwLock::new(table));
    // let mut wt1 = shared::WaveTable::new(xtable.clone(), f_sample_rate);
    // let mut wt2 = shared::WaveTable::new(xtable.clone(), f_sample_rate);
    // let mut wt3 = shared::WaveTable::new(xtable.clone(), f_sample_rate);
    // let mut wt4 = shared::WaveTable::new(xtable.clone(), f_sample_rate);
    // let mut wt5 = shared::WaveTable::new(xtable.clone(), f_sample_rate);
    // let mut wt6 = shared::WaveTable::new(xtable.clone(), f_sample_rate);


    let mut phasor = [0.0f32; SIZE];
    let mut lfo1 = [0.0f32; SIZE];
    let phase = phasor.phasor();
    let lfo1_t = lfo1.sine();
    let mut ph = owned::WaveTable::new(&phase, f_sample_rate);
    let mut lfo1_ph = owned::WaveTable::new(&lfo1_t, f_sample_rate);
    let mut imp = Dust::new(f_sample_rate);
    let mut out = 0.0;

    let mut gr1: Granulator2<16, {8*48000}> = Granulator2::new(
      // Buffer::new(f_sample_rate), 
      // Envelope::new(&bkr, f_sample_rate),
      brk,
      f_sample_rate,
    );

    let brk = BreakPoints::<3, 2>{
      values: [0.0, 1.0, 0.0], 
      durations: [0.2, 1.45], 
      curves: Some([0.2, 1.8])
    };

    let env = Envelope::new(&brk, f_sample_rate);
    let buf = buffer::Buffer::<{8*48000}>::new(f_sample_rate);
    
    let mut gr2: Granulator<16, {8*48000}> = Granulator::new(
      buf,
      env,
      f_sample_rate
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

        // if pos < SIZE && started {
        //   if let Ok(mut table) = xtable.write() {
        //     table[pos] = 0.0;
        //   }
        //   pos+=1;
        // }
        
        for sample in data {
          // Recieve sample from input stream
          *sample = match rx.try_recv() {
            Ok(s) => {
              if ch == 0 {
                if let Some(_) = gr1.record(s) {
                  out = 0.0;
                } else {
                    out = gr1.play::<Linear, Linear>(
                      ph.play::<Linear>(1.0/8.0, 1.0),
                      0.8,
                      (lfo1_ph.play::<Cubic>(0.05, 1.0) + 1.1) * 0.6,
                      0.51,
                      imp.play(0.1)
                    );
                    // out = wt1.play::<Linear>(freq, 1.0) * 0.2;
                    // out += wt2.play::<Linear>(freq*3.0, 1.0) * 0.2;
                    // out += wt3.play::<Linear>(freq*5.0, 1.0) * 0.2;
                    // out += wt4.play::<Linear>(freq*9.0, 1.0) * 0.2;
                    // out += wt5.play::<Linear>(freq*11.0*0.5, 1.0) * 0.2;
                    // out += wt6.play::<Linear>(freq*7.0, 1.0) * 0.16;
                  }
              } else {
                if let Some(_) = gr2.record(s) {
                  out = 0.0;
                } else {
                    out = gr2.play::<Linear, Linear>(
                      ph.play::<Linear>(1.0/8.0, 1.0),
                      0.8,
                      (lfo1_ph.play::<Cubic>(0.05, 1.0) + 1.1) * 0.6,
                      imp.play(0.1)
                    );
                    // out = wt1.play::<Linear>(freq, 1.0) * 0.2;
                    // out += wt2.play::<Linear>(freq*3.0, 1.0) * 0.2;
                    // out += wt3.play::<Linear>(freq*5.0, 1.0) * 0.2;
                    // out += wt4.play::<Linear>(freq*9.0, 1.0) * 0.2;
                    // out += wt5.play::<Linear>(freq*11.0*0.5, 1.0) * 0.2;
                    // out += wt6.play::<Linear>(freq*7.0, 1.0) * 0.16;
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
