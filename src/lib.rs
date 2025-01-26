/// Stream implementations using the [`cpal`](https://crates.io/crates/cpal) crate
pub mod cpal {
    mod input;
    pub use input::*;

    mod output;
    pub use output::*;
}

/// A sample
pub struct Sample(pub f32);

/// A stream of single-channel audio
pub trait Stream {
    /// Returns the sample rate of the stream
    fn sample_rate(&self) -> u32;
}

/// A stream that acts as an audio source
pub trait InputStream: Stream {
    /// Polls a new sample from the stream, if available
    /// # Returns
    /// The sample that is recorded, if available
    fn poll(&mut self) -> Vec<Sample>;

    /// Pauses the input capture of the stream in that
    /// no new samples are made available
    fn pause(&mut self);

    fn resume(&mut self);
}

/// A stream that acts as an audio sink
pub trait OutputStream: Stream {
    /// Pushes a new sample to the output
    /// # Arguments
    /// * `sample` - The sample to be played back
    fn push(&mut self, sample: Sample);
}
