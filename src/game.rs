use crate::wasm4;
use wasm4::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SelectPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    prev_gamepad: u8,
    selectPoint: SelectPoint,
    username: String,
    selectSwitch: bool
}

impl Game {
    pub fn new() -> Self {
      Self {
          username: String::from("BOB"),
          selectPoint: SelectPoint {x: 0, y: 0 },
          selectSwitch: false,
          prev_gamepad: 0
      }
    }
    
    pub fn input(&mut self) {
       let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);
if just_pressed & wasm4::BUTTON_1 != 0 {
            self.username.push_str("A");
        } 
        self.prev_gamepad = gamepad;
    }

    pub fn update(&mut self){
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

    pub fn selectUp(&self) {
       // self.snake.draw();
    }

    pub fn selectDown(&self) {}
    pub fn selectLeft(&self) {}
    pub fn selectRight(&self) {}

}
