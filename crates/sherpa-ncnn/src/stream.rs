//! Safe wrapper for sherpa-ncnn stream

use crate::recognizer::Recognizer;
use sherpa_ncnn_sys as sys;
use std::ffi::CStr;

/// Audio stream for recognition
pub struct Stream {
    pub(crate) ptr: *mut sys::SherpaNcnnStream,
}

// Safety: The C API is thread-safe for stream operations when used correctly
unsafe impl Send for Stream {}

impl Stream {
    pub(crate) fn new(ptr: *mut sys::SherpaNcnnStream) -> Self {
        Self { ptr }
    }

    /// Accept audio waveform samples
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate of the audio (typically 16000)
    /// * `samples` - Audio samples as f32, normalized to [-1, 1]
    pub fn accept_waveform(&mut self, sample_rate: f32, samples: &[f32]) {
        unsafe {
            sys::SherpaNcnnAcceptWaveform(
                self.ptr,
                sample_rate,
                samples.as_ptr(),
                samples.len() as i32,
            );
        }
    }

    /// Signal that no more audio will be provided
    pub fn input_finished(&mut self) {
        unsafe { sys::SherpaNcnnInputFinished(self.ptr); }
    }

    /// Check if the recognizer is ready to decode
    pub fn is_ready(&self, recognizer: &Recognizer) -> bool {
        unsafe { sys::SherpaNcnnIsReady(recognizer.ptr, self.ptr) != 0 }
    }

    /// Decode one frame
    pub fn decode(&mut self, recognizer: &Recognizer) {
        unsafe { sys::SherpaNcnnDecode(recognizer.ptr, self.ptr); }
    }

    /// Get the current recognition result
    pub fn get_result(&self, recognizer: &Recognizer) -> String {
        unsafe {
            let result_ptr = sys::SherpaNcnnGetResult(recognizer.ptr, self.ptr);
            if result_ptr.is_null() {
                return String::new();
            }

            let text = if (*result_ptr).text.is_null() {
                String::new()
            } else {
                CStr::from_ptr((*result_ptr).text)
                    .to_string_lossy()
                    .into_owned()
            };

            sys::SherpaNcnnDestroyResult(result_ptr);
            text
        }
    }

    /// Reset the stream for a new utterance
    pub fn reset(&mut self, recognizer: &Recognizer) {
        unsafe { sys::SherpaNcnnReset(recognizer.ptr, self.ptr); }
    }

    /// Check if an endpoint has been detected
    pub fn is_endpoint(&self, recognizer: &Recognizer) -> bool {
        unsafe { sys::SherpaNcnnIsEndpoint(recognizer.ptr, self.ptr) != 0 }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { sys::SherpaNcnnDestroyStream(self.ptr); }
        }
    }
}
