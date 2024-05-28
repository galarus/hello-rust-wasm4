#[cfg(feature = "buddy-alloc")]
mod alloc;
use std::sync::{Arc, Mutex};
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

pub static username: Mutex<String> = Mutex::new(String::new());   

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("what is your name?", 10, 10);
    text("A", 15, 25);
    text("B", 30, 25);
    text("C", 45, 25);
    text("D", 60, 25);
    text("E", 75, 25);
    text("F", 90, 25);
    text("G", 105, 25);
    text("H", 120, 25);
    text("I", 135, 25);

    text("J", 15, 40);
    text("K", 30, 40);
    text("L", 45, 40);
    text("M", 60, 40);
    text("N", 75, 40);
    text("O", 90, 40);
    text("P", 105, 40);
    text("Q", 120, 40);
    text("R", 135, 40);

    text("S", 15, 55);
    text("T", 30, 55);
    text("U", 45, 55);
    text("V", 60, 55);
    text("W", 75, 55);
    text("X", 90, 55);
    text("Y", 105, 55);
    text("Z", 120, 55);
    text("\\x84", 135, 55);
    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
        *username.lock().unwrap() = String::from("ABCDEF");
    }

    blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    text("Press X to blink", 16, 90);
    text(">: ", 16, 105);
    text(
&*username.lock().unwrap().clone()
        , 30, 105);
}
