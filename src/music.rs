use crate::Error;

pub struct Music;

impl Music {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        todo!()
    }

    pub fn from_wav(bytes: &'static [u8]) -> Result<Self, Error> {
        todo!()
    }

    pub fn play(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn pause(&mut self) {
        todo!()
    }

    pub fn resume(&mut self) {
        todo!()
    }

    pub fn stop(&mut self) {
        todo!()
    }

    pub fn set_volume(&mut self, volume: f32) {
        todo!()
    }

    pub fn is_playing(&self) -> bool {
        todo!()
    }
}
