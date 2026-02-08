//! Example: Streaming recognition with sherpa-ncnn
//!
//! This example demonstrates how to use the streaming API
//! to recognize audio in chunks, simulating real-time input.
//!
//! Usage:
//!   cargo run --example streaming -- <model_dir> <wav_file>

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
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
    };

    Ok((samples, spec.sample_rate))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <model_dir> <wav_file>", args[0]);
        std::process::exit(1);
    }

    let model_dir = &args[1];
    let wav_file = &args[2];

    // Read WAV file
    let (samples, sample_rate) = read_wav_file(wav_file)?;
    println!("Loaded {} samples at {} Hz", samples.len(), sample_rate);

    // Create recognizer with endpoint detection
    let config = RecognizerConfig::new(model_dir)
        .with_num_threads(4)
        .with_endpoint_detection(true);

    let recognizer = Recognizer::new(config)?;
    let mut stream = recognizer.create_stream()?;

    // Process audio in chunks (simulating streaming)
    let chunk_size = (sample_rate as usize) / 10; // 100ms chunks
    let mut last_result = String::new();

    for (i, chunk) in samples.chunks(chunk_size).enumerate() {
        stream.accept_waveform(sample_rate as f32, chunk);

        // Decode available frames
        while stream.is_ready(&recognizer) {
            stream.decode(&recognizer);
        }

        // Get partial result
        let result = stream.get_result(&recognizer);
        if result != last_result {
            println!(
                "[{:.1}s] {}",
                (i * chunk_size) as f32 / sample_rate as f32,
                result
            );
            last_result = result;
        }

        // Check for endpoint
        if stream.is_endpoint(&recognizer) {
            println!("--- Endpoint detected ---");
            stream.reset(&recognizer);
        }
    }

    // Signal end of audio
    stream.input_finished();

    // Decode remaining frames
    while stream.is_ready(&recognizer) {
        stream.decode(&recognizer);
    }

    let final_result = stream.get_result(&recognizer);
    println!();
    println!("Final result: {}", final_result);

    Ok(())
}
