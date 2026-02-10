//! Safe Rust bindings for sherpa-ncnn speech recognition
//!
//! This crate provides a safe, idiomatic Rust API for the sherpa-ncnn
//! speech recognition library.
//!
//! # Example
//!
//! ```no_run
//! use sherpa_ncnn::{Recognizer, RecognizerConfig};
//!
//! let config = RecognizerConfig::new("path/to/model");
//! let recognizer = Recognizer::new(config).unwrap();
//!
//! // Create a stream for recognition
//! let mut stream = recognizer.create_stream().unwrap();
//!
//! // Feed audio samples (16kHz, mono, f32)
//! let samples: Vec<f32> = vec![0.0; 16000]; // 1 second of silence
//! stream.accept_waveform(16000.0, &samples);
//! stream.input_finished();
//!
//! // Decode
//! while stream.is_ready(&recognizer) {
//!     stream.decode(&recognizer);
//! }
//!
//! let result = stream.get_result(&recognizer);
//! println!("Recognized: {}", result);
//! ```

mod error;
mod recognizer;
mod stream;
mod vad;

#[cfg(feature = "ncnn")]
pub mod ncnn;

pub use error::{Error, Result};
pub use recognizer::{DecoderConfig, FeatureConfig, ModelConfig, Recognizer, RecognizerConfig};
pub use stream::Stream;
pub use vad::{SpeechSegment, Vad, VadConfig};

/// Re-export the sys crate for advanced usage
pub use sherpa_ncnn_sys as sys;
