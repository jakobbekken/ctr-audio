#![no_std]

// Note: I think NDSP will require nightly feature allocator_api and prob alloc, will research

pub mod error;
pub mod music;
pub mod sfx;
pub mod wav;

pub use error::Error;
pub use music::Music;
pub use sfx::SoundEffect;
