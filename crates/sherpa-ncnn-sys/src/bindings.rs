// Pre-generated FFI bindings for sherpa-ncnn
// These are used when the C headers are not available

use std::os::raw::{c_char, c_float, c_int};

// ============================================================
// Opaque Types
// ============================================================

#[repr(C)]
#[derive(Debug)]
pub struct SherpaNcnnRecognizer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct SherpaNcnnStream {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct SherpaNcnnVoiceActivityDetector {
    _private: [u8; 0],
}

// ============================================================
// Configuration Structs
// ============================================================

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct SherpaNcnnFeatureExtractorConfig {
    pub sampling_rate: c_float,
    pub feature_dim: c_int,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SherpaNcnnModelConfig {
    pub encoder_param: *const c_char,
    pub encoder_bin: *const c_char,
    pub decoder_param: *const c_char,
    pub decoder_bin: *const c_char,
    pub joiner_param: *const c_char,
    pub joiner_bin: *const c_char,
    pub tokens: *const c_char,
    pub use_vulkan_compute: c_int,
    pub num_threads: c_int,
}

impl Default for SherpaNcnnModelConfig {
    fn default() -> Self {
        Self {
            encoder_param: std::ptr::null(),
            encoder_bin: std::ptr::null(),
            decoder_param: std::ptr::null(),
            decoder_bin: std::ptr::null(),
            joiner_param: std::ptr::null(),
            joiner_bin: std::ptr::null(),
            tokens: std::ptr::null(),
            use_vulkan_compute: 0,
            num_threads: 1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SherpaNcnnDecoderConfig {
    pub decoding_method: *const c_char,
    pub num_active_paths: c_int,
}

impl Default for SherpaNcnnDecoderConfig {
    fn default() -> Self {
        Self {
            decoding_method: std::ptr::null(),
            num_active_paths: 4,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SherpaNcnnRecognizerConfig {
    pub feat_config: SherpaNcnnFeatureExtractorConfig,
    pub model_config: SherpaNcnnModelConfig,
    pub decoder_config: SherpaNcnnDecoderConfig,
    pub enable_endpoint: c_int,
    pub rule1_min_trailing_silence: c_float,
    pub rule2_min_trailing_silence: c_float,
    pub rule3_min_utterance_length: c_float,
    pub hotwords_file: *const c_char,
    pub hotwords_score: c_float,
}

impl Default for SherpaNcnnRecognizerConfig {
    fn default() -> Self {
        Self {
            feat_config: SherpaNcnnFeatureExtractorConfig::default(),
            model_config: SherpaNcnnModelConfig::default(),
            decoder_config: SherpaNcnnDecoderConfig::default(),
            enable_endpoint: 1,
            rule1_min_trailing_silence: 2.4,
            rule2_min_trailing_silence: 1.2,
            rule3_min_utterance_length: 20.0,
            hotwords_file: std::ptr::null(),
            hotwords_score: 1.5,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SherpaNcnnResult {
    pub text: *const c_char,
    pub tokens: *const c_char,
    pub timestamps: *mut c_float,
    pub count: c_int,
}

// ============================================================
// VAD Structs
// ============================================================

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SherpaNcnnVadModelConfig {
    pub silero_vad_model: *const c_char,
    pub threshold: c_float,
    pub min_silence_duration: c_float,
    pub min_speech_duration: c_float,
    pub window_size: c_int,
    pub sample_rate: c_int,
    pub use_vulkan_compute: c_int,
    pub num_threads: c_int,
}

impl Default for SherpaNcnnVadModelConfig {
    fn default() -> Self {
        Self {
            silero_vad_model: std::ptr::null(),
            threshold: 0.5,
            min_silence_duration: 0.5,
            min_speech_duration: 0.25,
            window_size: 512,
            sample_rate: 16000,
            use_vulkan_compute: 0,
            num_threads: 1,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SherpaNcnnSpeechSegment {
    pub start: c_int,
    pub samples: *const c_float,
    pub n: c_int,
}

// ============================================================
// Extern Functions - ASR
// ============================================================

extern "C" {
    pub fn SherpaNcnnCreateRecognizer(
        config: *const SherpaNcnnRecognizerConfig,
    ) -> *mut SherpaNcnnRecognizer;

    pub fn SherpaNcnnDestroyRecognizer(recognizer: *mut SherpaNcnnRecognizer);

    pub fn SherpaNcnnCreateStream(
        recognizer: *const SherpaNcnnRecognizer,
    ) -> *mut SherpaNcnnStream;

    pub fn SherpaNcnnDestroyStream(stream: *mut SherpaNcnnStream);

    pub fn SherpaNcnnAcceptWaveform(
        stream: *mut SherpaNcnnStream,
        sample_rate: c_float,
        samples: *const c_float,
        n: c_int,
    );

    pub fn SherpaNcnnInputFinished(stream: *mut SherpaNcnnStream);

    pub fn SherpaNcnnIsReady(
        recognizer: *const SherpaNcnnRecognizer,
        stream: *const SherpaNcnnStream,
    ) -> c_int;

    pub fn SherpaNcnnDecode(
        recognizer: *const SherpaNcnnRecognizer,
        stream: *mut SherpaNcnnStream,
    );

    pub fn SherpaNcnnGetResult(
        recognizer: *const SherpaNcnnRecognizer,
        stream: *const SherpaNcnnStream,
    ) -> *const SherpaNcnnResult;

    pub fn SherpaNcnnDestroyResult(result: *const SherpaNcnnResult);

    pub fn SherpaNcnnReset(
        recognizer: *const SherpaNcnnRecognizer,
        stream: *mut SherpaNcnnStream,
    );

    pub fn SherpaNcnnIsEndpoint(
        recognizer: *const SherpaNcnnRecognizer,
        stream: *const SherpaNcnnStream,
    ) -> c_int;
}

// ============================================================
// Extern Functions - VAD
// ============================================================

extern "C" {
    pub fn SherpaNcnnCreateVoiceActivityDetector(
        config: *const SherpaNcnnVadModelConfig,
        buffer_size_in_seconds: c_float,
    ) -> *mut SherpaNcnnVoiceActivityDetector;

    pub fn SherpaNcnnDestroyVoiceActivityDetector(
        vad: *mut SherpaNcnnVoiceActivityDetector,
    );

    pub fn SherpaNcnnVoiceActivityDetectorAcceptWaveform(
        vad: *mut SherpaNcnnVoiceActivityDetector,
        samples: *const c_float,
        n: c_int,
    );

    pub fn SherpaNcnnVoiceActivityDetectorEmpty(
        vad: *const SherpaNcnnVoiceActivityDetector,
    ) -> c_int;

    pub fn SherpaNcnnVoiceActivityDetectorDetected(
        vad: *const SherpaNcnnVoiceActivityDetector,
    ) -> c_int;

    pub fn SherpaNcnnVoiceActivityDetectorFront(
        vad: *const SherpaNcnnVoiceActivityDetector,
    ) -> *const SherpaNcnnSpeechSegment;

    pub fn SherpaNcnnVoiceActivityDetectorPop(
        vad: *mut SherpaNcnnVoiceActivityDetector,
    );

    pub fn SherpaNcnnVoiceActivityDetectorClear(
        vad: *mut SherpaNcnnVoiceActivityDetector,
    );

    pub fn SherpaNcnnVoiceActivityDetectorReset(
        vad: *mut SherpaNcnnVoiceActivityDetector,
    );

    pub fn SherpaNcnnVoiceActivityDetectorFlush(
        vad: *mut SherpaNcnnVoiceActivityDetector,
    );

    pub fn SherpaNcnnDestroySpeechSegment(
        segment: *const SherpaNcnnSpeechSegment,
    );
}
