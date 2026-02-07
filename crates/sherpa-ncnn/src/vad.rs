//! Safe wrapper for sherpa-ncnn Voice Activity Detection (VAD)

use crate::error::{Error, Result};
use sherpa_ncnn_sys as sys;
use std::ffi::CString;
use std::path::Path;

/// Speech segment detected by VAD
#[derive(Debug, Clone)]
pub struct SpeechSegment {
    /// Start sample index
    pub start: i32,
    /// Audio samples
    pub samples: Vec<f32>,
}

/// VAD configuration
#[derive(Debug, Clone)]
pub struct VadConfig {
    /// Path to silero VAD model directory
    pub model_dir: String,
    /// Detection threshold (default: 0.5)
    pub threshold: f32,
    /// Minimum silence duration in seconds (default: 0.5)
    pub min_silence_duration: f32,
    /// Minimum speech duration in seconds (default: 0.25)
    pub min_speech_duration: f32,
    /// Window size in samples (default: 512)
    pub window_size: i32,
    /// Sample rate (default: 16000)
    pub sample_rate: i32,
    /// Use Vulkan compute (default: false)
    pub use_vulkan_compute: bool,
    /// Number of threads (default: 1)
    pub num_threads: i32,
}

impl VadConfig {
    /// Create a new VAD config with the given model path
    pub fn new<P: AsRef<Path>>(model_path: P) -> Self {
        Self {
            model_dir: model_path.as_ref().to_string_lossy().to_string(),
            threshold: 0.5,
            min_silence_duration: 0.5,
            min_speech_duration: 0.25,
            window_size: 512,
            sample_rate: 16000,
            use_vulkan_compute: false,
            num_threads: 1,
        }
    }

    /// Set the detection threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set the minimum silence duration
    pub fn with_min_silence_duration(mut self, duration: f32) -> Self {
        self.min_silence_duration = duration;
        self
    }

    /// Set the minimum speech duration
    pub fn with_min_speech_duration(mut self, duration: f32) -> Self {
        self.min_speech_duration = duration;
        self
    }

    /// Set the number of threads
    pub fn with_num_threads(mut self, num_threads: i32) -> Self {
        self.num_threads = num_threads;
        self
    }
}

/// Voice Activity Detector
pub struct Vad {
    ptr: *mut sys::SherpaNcnnVoiceActivityDetector,
    _model_path: CString,
}

// Safety: The C API is thread-safe for VAD operations
unsafe impl Send for Vad {}

impl Vad {
    /// Create a new VAD with the given configuration
    ///
    /// # Arguments
    /// * `config` - VAD configuration
    /// * `buffer_size_in_seconds` - Size of the internal buffer in seconds (default: 60.0)
    pub fn new(config: VadConfig, buffer_size_in_seconds: f32) -> Result<Self> {
        let c_model = CString::new(config.model_dir.clone())
            .map_err(|_| Error::InvalidConfig("Invalid model path".into()))?;

        let sys_config = sys::SherpaNcnnVadModelConfig {
            model_dir: c_model.as_ptr(),
            threshold: config.threshold,
            min_silence_duration: config.min_silence_duration,
            min_speech_duration: config.min_speech_duration,
            window_size: config.window_size,
            sample_rate: config.sample_rate,
            use_vulkan_compute: if config.use_vulkan_compute { 1 } else { 0 },
            num_threads: config.num_threads,
        };

        let ptr = unsafe {
            sys::SherpaNcnnCreateVoiceActivityDetector(&sys_config, buffer_size_in_seconds)
        };

        if ptr.is_null() {
            return Err(Error::VadCreation(
                "Failed to create VAD (null pointer returned)".into(),
            ));
        }

        Ok(Self {
            ptr,
            _model_path: c_model,
        })
    }

    /// Accept audio waveform samples
    ///
    /// # Arguments
    /// * `samples` - Audio samples as f32, normalized to [-1, 1]
    pub fn accept_waveform(&mut self, samples: &[f32]) {
        unsafe {
            sys::SherpaNcnnVoiceActivityDetectorAcceptWaveform(
                self.ptr,
                samples.as_ptr(),
                samples.len() as i32,
            );
        }
    }

    /// Check if there are no speech segments in the buffer
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SherpaNcnnVoiceActivityDetectorEmpty(self.ptr) != 0 }
    }

    /// Check if speech is currently detected
    pub fn is_speech_detected(&self) -> bool {
        unsafe { sys::SherpaNcnnVoiceActivityDetectorDetected(self.ptr) != 0 }
    }

    /// Pop the front speech segment from the buffer
    pub fn pop_front(&mut self) -> Option<SpeechSegment> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let segment_ptr = sys::SherpaNcnnVoiceActivityDetectorFront(self.ptr);
            if segment_ptr.is_null() {
                return None;
            }

            let segment = &*segment_ptr;
            let samples = std::slice::from_raw_parts(segment.samples, segment.n as usize).to_vec();
            let result = SpeechSegment {
                start: segment.start,
                samples,
            };

            sys::SherpaNcnnDestroySpeechSegment(segment_ptr);
            sys::SherpaNcnnVoiceActivityDetectorPop(self.ptr);

            Some(result)
        }
    }

    /// Get all speech segments from the buffer
    pub fn get_all_segments(&mut self) -> Vec<SpeechSegment> {
        let mut segments = Vec::new();
        while let Some(segment) = self.pop_front() {
            segments.push(segment);
        }
        segments
    }

    /// Flush the VAD to process any remaining audio
    pub fn flush(&mut self) {
        unsafe {
            sys::SherpaNcnnVoiceActivityDetectorFlush(self.ptr);
        }
    }

    /// Reset the VAD state
    pub fn reset(&mut self) {
        unsafe {
            sys::SherpaNcnnVoiceActivityDetectorReset(self.ptr);
        }
    }

    /// Clear the speech segment buffer
    pub fn clear(&mut self) {
        unsafe {
            sys::SherpaNcnnVoiceActivityDetectorClear(self.ptr);
        }
    }
}

impl Drop for Vad {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sys::SherpaNcnnDestroyVoiceActivityDetector(self.ptr);
            }
        }
    }
}
