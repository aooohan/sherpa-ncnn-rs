# sherpa-ncnn-rs

Rust bindings for [sherpa-ncnn](https://github.com/k2-fsa/sherpa-ncnn), a real-time speech recognition library based on ncnn.

## Features

- Real-time streaming speech recognition
- Offline speech recognition
- Voice Activity Detection (VAD)
- Cross-platform support (Android, iOS, macOS, Linux, Windows)
- Automatic download of pre-built native libraries
- Safe Rust API with proper memory management

## Supported Platforms

| Platform | Architectures |
|----------|---------------|
| Android | arm64-v8a, armeabi-v7a, x86_64, x86 |
| iOS | arm64, simulator-arm64, simulator-x86_64 |
| macOS | x86_64, arm64 |
| Linux | x86_64, aarch64, armv7 |
| Windows | x64 |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sherpa-ncnn = "0.1"
```

The library will automatically download pre-built native libraries for your platform.

### Manual Library Path

If you prefer to use your own build of sherpa-ncnn, set the environment variable:

```bash
export SHERPA_NCNN_LIB_PATH=/path/to/sherpa-ncnn/lib
```

## Usage

### Download a Model

First, download a pre-trained model from [sherpa-ncnn models](https://github.com/k2-fsa/sherpa-ncnn/releases):

```bash
# Example: Chinese + English bilingual model
wget https://github.com/k2-fsa/sherpa-ncnn/releases/download/models/sherpa-ncnn-streaming-zipformer-bilingual-zh-en-2023-02-13.tar.bz2
tar xvf sherpa-ncnn-streaming-zipformer-bilingual-zh-en-2023-02-13.tar.bz2
```

### Offline Transcription

```rust
use sherpa_ncnn::{Recognizer, RecognizerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create recognizer
    let config = RecognizerConfig::new("path/to/model")
        .with_num_threads(4)
        .with_decoding_method("greedy_search");

    let recognizer = Recognizer::new(config)?;

    // Load audio samples (16kHz, mono, f32 normalized to [-1, 1])
    let samples: Vec<f32> = load_audio_samples();

    // Transcribe
    let result = recognizer.transcribe(&samples, 16000.0)?;
    println!("Result: {}", result);

    Ok(())
}
```

### Streaming Recognition

```rust
use sherpa_ncnn::{Recognizer, RecognizerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RecognizerConfig::new("path/to/model")
        .with_num_threads(4)
        .with_endpoint_detection(true);

    let recognizer = Recognizer::new(config)?;
    let mut stream = recognizer.create_stream()?;

    // Process audio in chunks (e.g., from microphone)
    loop {
        let chunk: Vec<f32> = get_audio_chunk(); // Your audio source

        stream.accept_waveform(16000.0, &chunk);

        while stream.is_ready(&recognizer) {
            stream.decode(&recognizer);
        }

        let partial_result = stream.get_result(&recognizer);
        println!("Partial: {}", partial_result);

        // Check for endpoint (end of utterance)
        if stream.is_endpoint(&recognizer) {
            println!("Endpoint detected, resetting...");
            stream.reset(&recognizer);
        }
    }
}
```

### Voice Activity Detection (VAD)

```rust
use sherpa_ncnn::{Vad, VadConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = VadConfig::new("path/to/silero_vad.ncnn.bin")
        .with_threshold(0.5)
        .with_min_silence_duration(0.5)
        .with_min_speech_duration(0.25);

    let mut vad = Vad::new(config, 60.0)?;

    // Feed audio samples
    let samples: Vec<f32> = get_audio_samples();
    vad.accept_waveform(&samples);

    // Get speech segments
    vad.flush();
    for segment in vad.get_all_segments() {
        println!("Speech from sample {} with {} samples",
                 segment.start, segment.samples.len());
    }

    Ok(())
}
```

## Examples

Run the examples with:

```bash
# Offline transcription
cargo run -p sherpa-ncnn --example transcribe -- ./path/to/model ./audio.wav

# Streaming recognition
cargo run -p sherpa-ncnn --example streaming -- ./path/to/model ./audio.wav
```

## Building from Source

### Prerequisites

- Rust 1.70+
- CMake 3.14+
- C/C++ compiler

### Build

```bash
git clone https://github.com/aooohan/sherpa-ncnn-rs.git
cd sherpa-ncnn-rs
cargo build
```

### Cross-compilation for Android

```bash
# Install Android NDK and set ANDROID_NDK_HOME
rustup target add aarch64-linux-android
cargo build --target aarch64-linux-android
```

### Cross-compilation for iOS

```bash
rustup target add aarch64-apple-ios
cargo build --target aarch64-apple-ios
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SHERPA_NCNN_LIB_PATH` | Path to pre-built sherpa-ncnn libraries |
| `SHERPA_NCNN_RELEASE_URL` | Custom URL for downloading pre-built libraries |
| `SHERPA_BUILD_DEBUG` | Set to `1` to enable build debug logging |

## Crate Structure

```
sherpa-ncnn-rs/
├── crates/
│   ├── sherpa-ncnn-sys/    # Raw FFI bindings
│   └── sherpa-ncnn/        # Safe Rust wrapper
└── examples/
    ├── transcribe.rs       # Offline transcription example
    └── streaming.rs        # Streaming recognition example
```

## Related Projects

- [sherpa-ncnn](https://github.com/k2-fsa/sherpa-ncnn) - Original C++ implementation
- [sherpa-rs](https://github.com/thewh1teagle/sherpa-rs) - Rust bindings for sherpa-onnx

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [k2-fsa/sherpa-ncnn](https://github.com/k2-fsa/sherpa-ncnn) - The underlying speech recognition engine
- [Tencent/ncnn](https://github.com/Tencent/ncnn) - High-performance neural network inference framework
