#![windows_subsystem = "windows"]

mod quack;
use crate::quack::quack;

fn main() {
    unsafe {
        quack();
    }
}
