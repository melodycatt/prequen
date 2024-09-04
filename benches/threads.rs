use image::open;
use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use core::num;
use std::time::Instant;
use std::f64::consts::PI;
use rayon::prelude::*;
use std::time::Duration;
use rodio::{OutputStream, Sink, Source};

fn bench(num_threads: usize) {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads) // For example, use 100 threads
        .build()
        .unwrap();  

    pool.install(|| {
        let image = open("./cat.png").unwrap().to_rgb8();
        let rgb: Vec<u8> = image.into_raw();
        let mut changes: Vec<Vec<u8>> = vec![];
        for color in rgb.chunks_exact(3) {
            changes.push(color.try_into().unwrap());
        }
        //let start = Instant::now();
        // Define a Waveform with 200Hz sampling rate and three function components,
        // choosing f32 as the output type:
        //let wave = gen_waveform_is(44100, values, 0.005);
        //println!("{:?}", wave)
        //let _wav = gen_waveform(44100, values, 0.005);//0.00013
        let sample_rate: u32 = 44100;
        let change_length: f64 = 0.005;
            //let mut wav = Vec::with_capacity((sample_rate as f32 * changes.len() as f32 * change_length) as usize);
        let change_interval = change_length * sample_rate as f64;
        //println!("{}", change_interval);

        //let mut change_i = 0;
        let sample_rate_f64 = sample_rate as f64;
        let total_samples = (sample_rate_f64 * changes.len() as f64 * change_length) as usize;

        let wav: Vec<f64> = (0..total_samples).into_par_iter().map(|i| {
        //for i in 0..total_samples  {
            let t = i as f64 / sample_rate_f64;
            let mut change_i = (i as f64 / change_interval).floor() as usize;
            if change_i >= changes.len() {
                //println!("{} ci {} i |||| {} / {} iasf32 {} cin {} iasf64", change_i, i, (i as f64 / change_interval), i as f32, change_interval, i as f64);
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
            (2.0 * PI * f * 500.0 * t).sin() / 20.0
            //wav.push(sample);
        //}
        }).collect();
        //println!("{} length in secs", wav.len() / sample_rate as usize);
        let _ = wav.iter().map(|&x| x as f32).collect::<Vec<f32>>();
        //let dur = start.elapsed();
        //println!("Time with {} threads: {:?}", num_threads, dur)
    })
}

fn gen_waveform(sample_rate: u32, changes: Vec<Vec<u8>>, change_length: f64) -> Vec<f32> {
    //let mut wav = Vec::with_capacity((sample_rate as f32 * changes.len() as f32 * change_length) as usize);
    let change_interval = change_length * sample_rate as f64;
    //println!("{}", change_interval);

    //let mut change_i = 0;
    let sample_rate_f64 = sample_rate as f64;
    let total_samples = (sample_rate_f64 * changes.len() as f64 * change_length) as usize;

    let wav: Vec<f64> = (0..total_samples).into_par_iter().map(|i| {
    //for i in 0..total_samples  {
        let t = i as f64 / sample_rate_f64;
        let mut change_i = (i as f64 / change_interval).floor() as usize;
        if change_i >= changes.len() {
            //println!("{} ci {} i |||| {} / {} iasf32 {} cin {} iasf64", change_i, i, (i as f64 / change_interval), i as f32, change_interval, i as f64);
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
        (2.0 * PI * f * 500.0 * t).sin() / 20.0
        //wav.push(sample);
    //}
    }).collect();
    //println!("{} length in secs", wav.len() / sample_rate as usize);
    wav.iter().map(|&x| x as f32).collect()
}

fn bench_threads(c: &mut Criterion) {
    let thread_counts = vec![16, 64]; // Adjust these as needed
    let mut group = c.benchmark_group("thread_counts");
    group.significance_level(0.1).sample_size(40).measurement_time(Duration::from_secs(30)).warm_up_time(Duration::from_millis(7500));

    for &count in thread_counts.iter() {
        group.bench_function(&format!("threads_{}", count), |b| {
            b.iter(|| bench(count));
        });
    }
    group.finish()
}

criterion_group!(benches, bench_threads);
criterion_main!(benches);
