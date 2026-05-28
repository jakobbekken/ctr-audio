use crate::Error;
use ctru::services::ndsp::Ndsp;

pub struct SoundEffect;

impl SoundEffect {
    pub fn from_wav(bytes: &'static [u8]) -> Result<Self, Error> {
        todo!()
    }

    pub fn play(&mut self, ndsp: &Ndsp) -> Result<(), Error> {
        todo!()
    }

    pub fn stop(&mut self) {
        todo!()
    }

    pub fn set_volume(&mut self, volume: f32) {
        todo!()
    }
}
