//#![feature(f128)]

use image::open;
use std::f64::consts::PI;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::time::Duration;
use rodio::{OutputStream, Sink, Source};

struct SineWave {
    samples: Vec<f32>,
    sample_rate: u32,
    position: usize,
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.samples.len() {
            let sample = self.samples[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.position)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        1 // Mono
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32,
        ))
    }
}

fn gen_waveform(sample_rate_f64: f64, changes: Vec<Vec<u8>>, change_length: f64) -> Vec<f32> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(16) // For example, use 100 threads
        .build()
        .unwrap();  

    pool.install(|| {
        //let mut wav = Vec::with_capacity((sample_rate as f32 * changes.len() as f32 * change_length) as usize);
        let change_interval = change_length * sample_rate_f64;
        
        //let mut change_i = 0;
        //let sample_rate_f64 = sample_rate as f64;
        let total_samples = (sample_rate_f64 * changes.len() as f64 * change_length) as usize;
        println!("{} {}", total_samples, usize::MAX);
    
        let mut precomputed_frequencies: Vec<f64> = changes.windows(2).map(|pair| {
            pair[0]
                .iter()
                .zip(pair[1].iter())
                .map(|(x, y)| (*x as u16 * *y as u16) as f64)
                .sum::<f64>()
        }).collect();
        precomputed_frequencies.push(changes[0]
            .iter().zip(changes[changes.len() - 1].iter())
            .map(|(x, y)| (*x as u16 * *y as u16) as f64).sum::<f64>()
        );

        (0..total_samples).into_par_iter().map(|i| {
        //for i in 0..total_samples  {
            let t = i as f64 / sample_rate_f64;
            let mut change_i = (i as f64 / change_interval).floor() as usize;
            if change_i >= changes.len() {
                println!("{} ci {} i |||| {} / {} iasf32 {} cin {} iasf64", change_i, i, (i as f64 / change_interval), i as f32, change_interval, i as f64);
                change_i = change_i.min(changes.len() - 1)
            }
            //println!("{} changei {} i {:?} change ", change_i, i, changes[change_i]);
            /*if change_i > 0 { 
                println!("{:?} zip ", changes[change_i]
                .iter().zip(changes[change_i - 1].iter())
                .map(|(x, y)| {//println!("{} x {} y", x, y); 
                (*x as u16 * *y as u16) as f64}).sum::<f64>()) 
            }; //FROM SUM*/
            /*let f = 
                if change_i == 0 { 
                    changes[0]
                        .iter().zip(changes[changes.len() - 1].iter())
                        .map(|(x, y)| (*x as u16 * *y as u16) as f64).sum::<f64>()
                } 
                else { 
                    changes[change_i]
                        .iter().zip(changes[change_i - 1].iter())
                        .map(|(x, y)| {//println!("{} x {} y", x, y); 
                        (*x as u16 * *y as u16) as f64}).sum::<f64>() 
                };*/
            //let sample = (2.0 * PI * f * 500.0 * t).sin() / 20.0;
            (2.0 * PI * precomputed_frequencies[change_i] * 1.0 * t).sin() as f32 / 2.0
            //wav.push(sample);
        //}
        }).collect()
        //wav.iter().map(|&x| x as f32).collect()
    })
}
fn _gen_waveform_is(sample_rate: u32, changes: Vec<Vec<u8>>, change_length: f64) -> Vec<(f32, usize)> {
    //let mut wav = Vec::with_capacity((sample_rate as f32 * changes.len() as f32 * change_length) as usize);
    let change_interval = change_length * sample_rate as f64;
    println!("{}", change_interval);

    //let mut change_i = 0;
    let sample_rate_f64 = sample_rate as f64;
    let total_samples = (sample_rate_f64 * changes.len() as f64 * change_length) as usize;

    let wav: Vec<(f64, usize)> = (0..total_samples).into_par_iter().map(|i| {
    //for i in 0..total_samples  {
        let t = i as f64 / sample_rate_f64;
        let mut change_i = (i as f64 / change_interval).floor() as usize;
        if change_i >= changes.len() {
            println!("{} ci {} i |||| {} / {} iasf32 {} cin {} iasf64", change_i, i, (i as f64 / change_interval), i as f32, change_interval, i as f64);
            change_i = change_i.min(changes.len() - 1)
        }
        //println!("{} changei {} i {:?} change ", change_i, i, changes[change_i]);
        //if change_i > 0 { println!("{:?} zip ", changes[change_i].iter().zip(changes[change_i - 1].iter()).map(|(x, y)| {println!("{} x {} y", x, y); (*x as u16 * *y as u16) as f32}).sum::<f32>() ) }; //FROM SUM

        let f = 
            if change_i == 0 { 
                changes[0]
                    .iter().zip(changes[changes.len() - 1].iter())
                    .map(|(x, y)| (*x as u16 * *y as u16) as f64).sum::<f64>()
            } 
            else { 
                changes[change_i]
                    .iter().zip(changes[change_i - 1].iter())
                    .map(|(x, y)| {//println!("{} x {} y", x, y); 
                    (*x as u16 * *y as u16) as f64}).sum::<f64>() 
            };
        //let sample = (2.0 * PI * f * 500.0 * t).sin() / 20.0;
        ((2.0 * PI * f / 100.0 * t).sin() / 20.0, i)
        //wav.push(sample);
    //}
    }).collect();
    println!("{} length in secs", wav.len() / sample_rate as usize);
    wav.iter().map(|&x| (x.0 as f32, x.1)).collect()
}

fn play_waveform(samples: Vec<f32>, sample_rate: u32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    println!("playing?");
    let sine_wave = SineWave {
        samples,
        sample_rate,
        position: 0,
    };

    sink.append(sine_wave);
    sink.sleep_until_end();
}

fn main() {
    let image = open("./cat.png").unwrap().to_rgba8();
    let rgb: Vec<u8> = image.into_raw();
    let mut values: Vec<Vec<u8>> = vec![];
    for color in rgb.chunks_exact(4) {
        values.push(color.try_into().unwrap());
        println!("{:?}", color)
    }
    // Define a Waveform with 200Hz sampling rate and three function components,
    // choosing f32 as the output type:
    //let wave = gen_waveform_is(44100, values, 0.005);
    //println!("{:?}", wave)
    let wav = gen_waveform(44100.0, values, 0.5);//0.00013
    println!("{} length in secs", wav.len() / 44100 as usize);
    std::thread::sleep(Duration::from_secs(2));
    play_waveform(wav, 44100)
}
