use std::{
    sync::mpsc::{self, Sender},
    time::Duration,
};

use cpal::traits::{DeviceTrait, StreamTrait};

use crate::{OutputStream, Sample, Stream};

pub struct CPALOutputStream {
    config: cpal::StreamConfig,
    stream: cpal::Stream,
    sender: Sender<Sample>,
}

impl CPALOutputStream {
    pub fn new(
        device: cpal::Device,
        config: cpal::StreamConfig,
        timeout: Option<Duration>,
    ) -> Result<Self, cpal::BuildStreamError> {
        let (sender, receiver) = mpsc::channel::<Sample>();

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _info| {
                for sample_container in data.iter_mut() {
                    if let Some(sample) = receiver.try_iter().next() {
                        *sample_container = sample.0;
                    }
                }

                // We clear the remaining data from the pipe
                // so that we don't introduce delay
                for _ in data.iter_mut() {}
            },
            |_info| {},
            timeout,
        )?;

        stream.play().unwrap();

        Ok(Self {
            config,
            stream,
            sender,
        })
    }
}

impl Stream for CPALOutputStream {
    fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }
}

impl OutputStream for CPALOutputStream {
    fn push(&mut self, sample: Sample) {
        self.sender.send(sample).unwrap();
    }
}
