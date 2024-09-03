use image:: {
    open,
    DynamicImage,
    *   
};
use std::f32::consts::PI;

use rodio::{OutputStream, Source};
use std::time::Duration;

struct SineWaveSource {
    waveform: Vec<f32>,
    sample_rate: u32,
    current_sample: usize,
}

impl SineWaveSource {
    fn new(waveform: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            waveform,
            sample_rate,
            current_sample: 0,
        }
    }
}

impl Iterator for SineWaveSource {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample < self.waveform.len() {
            let sample = self.waveform[self.current_sample];
            self.current_sample += 1;
            Some((sample * i16::MAX as f32) as i16)
        } else {
            None
        }
    }
}

impl Source for SineWaveSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.waveform.len())
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.waveform.len() as f32 / self.sample_rate as f32))
    }
}


fn gen_waveform(sample_rate: u32, changes: Vec<Vec<u8>>, change_length: f32) -> Vec<f32> {
    let mut wav = Vec::<f32>::new();

    let mut change_i = 0;

    for i in 0..(sample_rate as f32 * changes.len() as f32 * change_length) as usize  {
        let t = i as f32 / sample_rate as f32;
        if i as f32 >= (change_i + 1) as f32 * change_length * sample_rate as f32 {
            change_i += 1;
        }
        println!("{} changei {} i {:?} change ", change_i, i, changes[change_i]);
        if change_i > 0 { println!("{:?} zip ", changes[change_i].iter().zip(changes[change_i - 1].iter()).map(|(x, y)| {println!("{} x {} y", x, y); (x * y) as f32}).sum::<f32>() ) }; //FROM SUM

        let f = 
            if change_i == 0 { 
                changes[0].iter().zip(vec![0, 0, 0].iter()).map(|(x, y)| (x * y) as f32).sum::<f32>()
            } 
            else { 
                changes[change_i].iter().zip(changes[change_i - 1].iter()).map(|(x, y)| {println!("{} x {} y", x, y); (x * y) as f32}).sum::<f32>() 
            };
        let sample = (2.0 * PI * f + t).sin();
        wav.push(sample);
    }
    wav
}

fn play_waveform(waveform: Vec<f32>, sample_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Get an output stream
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Create a sine wave source
    let source = SineWaveSource::new(waveform, sample_rate);

    let length = source.waveform.len() as f32 / sample_rate as f32;

    // Play the sound
    stream_handle.play_raw(source.convert_samples())?;

    // Keep the program running until the sound is done playing
    std::thread::sleep(Duration::from_secs_f32(length));
    
    Ok(())
}

fn main() {
    let image = open(".\\cat.png").unwrap().to_rgb8();
    let rgb: Vec<u8> = image.into_raw();
    let mut values: Vec<Vec<u8>> = vec![];
    for color in rgb.chunks_exact(3) {
        values.push(color.try_into().unwrap());
    }
    // Define a Waveform with 200Hz sampling rate and three function components,
    // choosing f32 as the output type:

    let wav = gen_waveform(44100, values, 0.1);
    println!("{:?}", wav);

    match play_waveform(wav, 44100) {
        Ok(_) => println!("Playing waveform"),
        Err(e) => eprintln!("Failed to play waveform: {}", e),
    }

}
