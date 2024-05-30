use crate::wasm4;
use wasm4::*;

const select_box: [u8; 8] = [
    0b00000000,
    0b01111110,
    0b01111110,
    0b01111110,
    0b01111110,
    0b01111110,
    0b01111110,
    0b00000000,
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SelectPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    prev_gamepad: u8,
    select_point: SelectPoint,
    username: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            username: String::from("BOB"),
            select_point: SelectPoint { x: 0, y: 0 },
            prev_gamepad: 0,
        }
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);
        if just_pressed & wasm4::BUTTON_1 != 0 {
            self.username.push_str("A");
        }

        if just_pressed & wasm4::BUTTON_LEFT != 0 {
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
        }
        if just_pressed & wasm4::BUTTON_UP != 0 {
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
        }

        self.prev_gamepad = gamepad;
    }

    pub fn update(&mut self) {
        self.input();

        blit(&select_box,
             15 * (self.select_point.x+1),
             25 + 15 * self.select_point.y,
             8, 8, BLIT_1BPP);


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
        text(b"\x84", 135, 55);

        text("Press X to blink", 16, 90);
        text(">: ", 16, 105);
        text(&self.username, 30, 105);
    }

  //  pub fn select_up(&self) {}

  //  pub fn select_down(&self) {}
  //  pub fn select_left(&self) {}
  //  pub fn select_right(&self) {}
}
