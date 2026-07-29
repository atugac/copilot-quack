use windows::Win32::Media::Audio::{PlaySoundA, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCSTR;

const QUACK: &[u8] = include_bytes!("quack.wav");

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn quack() {
    let _ = PlaySoundA(
        PCSTR::from_raw(QUACK.as_ptr()),
        None,
        SND_MEMORY | SND_NODEFAULT
    );
}