//! Example: Transcribe a WAV file using sherpa-ncnn
//!
//! Usage:
//!   cargo run --example transcribe -- <model_dir> <wav_file>

use sherpa_ncnn::{Recognizer, RecognizerConfig};
use std::env;

fn read_wav_file(path: &str) -> Result<(Vec<f32>, u32), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    if spec.channels != 1 {
        return Err("Only mono audio is supported".into());
    }

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 / max_val)
                .collect()
        }
        hound::SampleFormat::Float => {
            reader.samples::<f32>().map(|s| s.unwrap()).collect()
        }
    };

    Ok((samples, spec.sample_rate))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <model_dir> <wav_file>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} ./sherpa-ncnn-streaming-zipformer-bilingual-zh-en-2023-02-13 test.wav", args[0]);
        std::process::exit(1);
    }

    let model_dir = &args[1];
    let wav_file = &args[2];

    println!("Model directory: {}", model_dir);
    println!("WAV file: {}", wav_file);

    // Read WAV file
    let (samples, sample_rate) = read_wav_file(wav_file)?;
    println!("Sample rate: {} Hz", sample_rate);
    println!("Duration: {:.2} seconds", samples.len() as f32 / sample_rate as f32);

    // Create recognizer
    let config = RecognizerConfig::new(model_dir)
        .with_num_threads(4)
        .with_decoding_method("greedy_search");

    println!("Creating recognizer...");
    let recognizer = Recognizer::new(config)?;

    // Transcribe
    println!("Transcribing...");
    let result = recognizer.transcribe(&samples, sample_rate as f32)?;

    println!();
    println!("Result: {}", result);

    Ok(())
}
