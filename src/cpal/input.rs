use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use cpal::traits::{DeviceTrait, StreamTrait};

use crate::{InputStream, Sample, Stream};

pub struct CPALInputStream {
    config: cpal::StreamConfig,
    stream: cpal::Stream,
    receiver: Receiver<Sample>,
}

impl CPALInputStream {
    pub fn new(
        device: cpal::Device,
        config: cpal::StreamConfig,
        timeout: Option<Duration>,
    ) -> Result<Self, cpal::BuildStreamError> {
        let (sender, receiver) = mpsc::channel();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _info| {
                for sample in data {
                    if let Err(e) = sender.send(Sample(*sample)) {
                        eprintln!("Stream capture error: {e}")
                    }
                }
            },
            |_info| {},
            timeout,
        )?;

        stream.play().unwrap();

        Ok(Self {
            config,
            stream,
            receiver,
        })
    }
}

impl Stream for CPALInputStream {
    fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }
}

impl InputStream for CPALInputStream {
    fn poll(&mut self) -> Option<crate::Sample> {
        self.receiver.iter().next()
    }

    fn pause(&mut self) {
        self.stream.pause().unwrap();
    }

    fn resume(&mut self) {
        self.stream.play().unwrap();
    }
}
