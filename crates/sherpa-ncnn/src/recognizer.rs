//! Safe wrapper for sherpa-ncnn recognizer

use crate::error::{Error, Result};
use crate::stream::Stream;
use sherpa_ncnn_sys as sys;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

/// Feature extractor configuration
#[derive(Debug, Clone)]
pub struct FeatureConfig {
    /// Sample rate of audio (default: 16000)
    pub sample_rate: f32,
    /// Feature dimension (default: 80)
    pub feature_dim: i32,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000.0,
            feature_dim: 80,
        }
    }
}

/// Model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// Path to encoder .param file
    pub encoder_param: String,
    /// Path to encoder .bin file
    pub encoder_bin: String,
    /// Path to decoder .param file
    pub decoder_param: String,
    /// Path to decoder .bin file
    pub decoder_bin: String,
    /// Path to joiner .param file
    pub joiner_param: String,
    /// Path to joiner .bin file
    pub joiner_bin: String,
    /// Path to tokens.txt
    pub tokens: String,
    /// Use Vulkan compute (default: false)
    pub use_vulkan_compute: bool,
    /// Number of threads (default: 4)
    pub num_threads: i32,
}

impl ModelConfig {
    /// Create model config from a model directory
    ///
    /// Expects the directory to contain:
    /// - encoder_jit_trace-pnnx.ncnn.param
    /// - encoder_jit_trace-pnnx.ncnn.bin
    /// - decoder_jit_trace-pnnx.ncnn.param
    /// - decoder_jit_trace-pnnx.ncnn.bin
    /// - joiner_jit_trace-pnnx.ncnn.param
    /// - joiner_jit_trace-pnnx.ncnn.bin
    /// - tokens.txt
    pub fn from_dir<P: AsRef<Path>>(model_dir: P) -> Self {
        let dir = model_dir.as_ref().to_string_lossy();
        Self {
            encoder_param: format!("{}/encoder_jit_trace-pnnx.ncnn.param", dir),
            encoder_bin: format!("{}/encoder_jit_trace-pnnx.ncnn.bin", dir),
            decoder_param: format!("{}/decoder_jit_trace-pnnx.ncnn.param", dir),
            decoder_bin: format!("{}/decoder_jit_trace-pnnx.ncnn.bin", dir),
            joiner_param: format!("{}/joiner_jit_trace-pnnx.ncnn.param", dir),
            joiner_bin: format!("{}/joiner_jit_trace-pnnx.ncnn.bin", dir),
            tokens: format!("{}/tokens.txt", dir),
            use_vulkan_compute: false,
            num_threads: 4,
        }
    }
}

/// Decoder configuration
#[derive(Debug, Clone)]
pub struct DecoderConfig {
    /// Decoding method: "greedy_search" or "modified_beam_search"
    pub decoding_method: String,
    /// Number of active paths for beam search (default: 4)
    pub num_active_paths: i32,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            decoding_method: "greedy_search".to_string(),
            num_active_paths: 4,
        }
    }
}

/// Recognizer configuration
#[derive(Debug, Clone)]
pub struct RecognizerConfig {
    /// Feature extractor configuration
    pub feature_config: FeatureConfig,
    /// Model configuration
    pub model_config: ModelConfig,
    /// Decoder configuration
    pub decoder_config: DecoderConfig,
    /// Enable endpoint detection (default: true)
    pub enable_endpoint: bool,
    /// Rule1: min trailing silence for endpoint (default: 2.4)
    pub rule1_min_trailing_silence: f32,
    /// Rule2: min trailing silence for endpoint (default: 1.2)
    pub rule2_min_trailing_silence: f32,
    /// Rule3: min utterance length for endpoint (default: 20.0)
    pub rule3_min_utterance_length: f32,
    /// Path to hotwords file (optional)
    pub hotwords_file: Option<String>,
    /// Hotwords score (default: 1.5)
    pub hotwords_score: f32,
}

impl RecognizerConfig {
    /// Create a new recognizer config from a model directory
    pub fn new<P: AsRef<Path>>(model_dir: P) -> Self {
        Self {
            feature_config: FeatureConfig::default(),
            model_config: ModelConfig::from_dir(model_dir),
            decoder_config: DecoderConfig::default(),
            enable_endpoint: true,
            rule1_min_trailing_silence: 2.4,
            rule2_min_trailing_silence: 1.2,
            rule3_min_utterance_length: 20.0,
            hotwords_file: None,
            hotwords_score: 1.5,
        }
    }

    /// Set the number of threads
    pub fn with_num_threads(mut self, num_threads: i32) -> Self {
        self.model_config.num_threads = num_threads;
        self
    }

    /// Set the decoding method
    pub fn with_decoding_method(mut self, method: &str) -> Self {
        self.decoder_config.decoding_method = method.to_string();
        self
    }

    /// Enable or disable endpoint detection
    pub fn with_endpoint_detection(mut self, enable: bool) -> Self {
        self.enable_endpoint = enable;
        self
    }
}

/// Speech recognizer
pub struct Recognizer {
    pub(crate) ptr: *mut sys::SherpaNcnnRecognizer,
    // Keep CStrings alive for the lifetime of the recognizer
    _strings: Vec<CString>,
}

// Safety: The C API is thread-safe for recognizer operations
unsafe impl Send for Recognizer {}
unsafe impl Sync for Recognizer {}

impl Recognizer {
    /// Create a new recognizer with the given configuration
    pub fn new(config: RecognizerConfig) -> Result<Self> {
        let mut strings = Vec::new();

        // Convert strings to CStrings
        let c_encoder_param = CString::new(config.model_config.encoder_param.clone())
            .map_err(|_| Error::InvalidConfig("Invalid encoder_param path".into()))?;
        let c_encoder_bin = CString::new(config.model_config.encoder_bin.clone())
            .map_err(|_| Error::InvalidConfig("Invalid encoder_bin path".into()))?;
        let c_decoder_param = CString::new(config.model_config.decoder_param.clone())
            .map_err(|_| Error::InvalidConfig("Invalid decoder_param path".into()))?;
        let c_decoder_bin = CString::new(config.model_config.decoder_bin.clone())
            .map_err(|_| Error::InvalidConfig("Invalid decoder_bin path".into()))?;
        let c_joiner_param = CString::new(config.model_config.joiner_param.clone())
            .map_err(|_| Error::InvalidConfig("Invalid joiner_param path".into()))?;
        let c_joiner_bin = CString::new(config.model_config.joiner_bin.clone())
            .map_err(|_| Error::InvalidConfig("Invalid joiner_bin path".into()))?;
        let c_tokens = CString::new(config.model_config.tokens.clone())
            .map_err(|_| Error::InvalidConfig("Invalid tokens path".into()))?;
        let c_decoding_method = CString::new(config.decoder_config.decoding_method.clone())
            .map_err(|_| Error::InvalidConfig("Invalid decoding_method".into()))?;

        let c_hotwords = config.hotwords_file
            .as_ref()
            .map(|s| CString::new(s.clone()).ok())
            .flatten();

        let sys_config = sys::SherpaNcnnRecognizerConfig {
            feat_config: sys::SherpaNcnnFeatureExtractorConfig {
                sampling_rate: config.feature_config.sample_rate,
                feature_dim: config.feature_config.feature_dim,
            },
            model_config: sys::SherpaNcnnModelConfig {
                encoder_param: c_encoder_param.as_ptr(),
                encoder_bin: c_encoder_bin.as_ptr(),
                decoder_param: c_decoder_param.as_ptr(),
                decoder_bin: c_decoder_bin.as_ptr(),
                joiner_param: c_joiner_param.as_ptr(),
                joiner_bin: c_joiner_bin.as_ptr(),
                tokens: c_tokens.as_ptr(),
                use_vulkan_compute: if config.model_config.use_vulkan_compute { 1 } else { 0 },
                num_threads: config.model_config.num_threads,
            },
            decoder_config: sys::SherpaNcnnDecoderConfig {
                decoding_method: c_decoding_method.as_ptr(),
                num_active_paths: config.decoder_config.num_active_paths,
            },
            enable_endpoint: if config.enable_endpoint { 1 } else { 0 },
            rule1_min_trailing_silence: config.rule1_min_trailing_silence,
            rule2_min_trailing_silence: config.rule2_min_trailing_silence,
            rule3_min_utterance_length: config.rule3_min_utterance_length,
            hotwords_file: c_hotwords.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
            hotwords_score: config.hotwords_score,
        };

        // Keep CStrings alive
        strings.push(c_encoder_param);
        strings.push(c_encoder_bin);
        strings.push(c_decoder_param);
        strings.push(c_decoder_bin);
        strings.push(c_joiner_param);
        strings.push(c_joiner_bin);
        strings.push(c_tokens);
        strings.push(c_decoding_method);
        if let Some(s) = c_hotwords {
            strings.push(s);
        }

        let ptr = unsafe { sys::SherpaNcnnCreateRecognizer(&sys_config) };

        if ptr.is_null() {
            return Err(Error::RecognizerCreation(
                "Failed to create recognizer (null pointer returned)".into()
            ));
        }

        Ok(Self { ptr, _strings: strings })
    }

    /// Create a new stream for recognition
    pub fn create_stream(&self) -> Result<Stream> {
        let ptr = unsafe { sys::SherpaNcnnCreateStream(self.ptr) };

        if ptr.is_null() {
            return Err(Error::StreamCreation(
                "Failed to create stream (null pointer returned)".into()
            ));
        }

        Ok(Stream::new(ptr))
    }

    /// Transcribe audio samples directly (convenience method)
    ///
    /// This creates a stream, feeds the audio, and returns the result.
    /// For streaming recognition, use `create_stream()` instead.
    pub fn transcribe(&self, samples: &[f32], sample_rate: f32) -> Result<String> {
        let mut stream = self.create_stream()?;
        stream.accept_waveform(sample_rate, samples);
        stream.input_finished();

        while stream.is_ready(self) {
            stream.decode(self);
        }

        Ok(stream.get_result(self))
    }
}

impl Drop for Recognizer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { sys::SherpaNcnnDestroyRecognizer(self.ptr); }
        }
    }
}
