use crate::Error;
use ctru::services::ndsp::AudioFormat;

pub struct WavData {
    pub sample_rate: u32,
    pub format: AudioFormat,
    pub data: &'static [u8],
}

impl WavData {
    pub fn parse(bytes: &'static [u8]) -> Result<WavData, Error> {
        todo!()
    }
}
