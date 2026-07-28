use windows::core::PCSTR;
use windows::Win32::Media::Audio::{PlaySoundA, SND_MEMORY, SND_NODEFAULT};

const QUACK: &[u8] = include_bytes!("quack.wav");

fn main() {
    unsafe {
        let _ = PlaySoundA(
            PCSTR::from_raw(QUACK.as_ptr()),
            None,
            SND_MEMORY | SND_NODEFAULT
        );
    }
}
