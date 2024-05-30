use crate::wasm4;
use wasm4::*;

const NAME_LIMIT: usize = 12;

const SPACE_BAR: [u8; 8] = [
    0b11111111,
    0b11111111,
    0b11111111,
    0b11111111,
    0b11111111,
    0b11111111,
    0b01111110,
    0b00000000,
];


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SelectPoint {
    pub x: i32,
    pub y: i32,
}

fn get_char_from_xy(x: u8, y: u8) -> String{
    let ascii_offset = y * 9 + x;
    let ascii_num = 65 + ascii_offset;
    let utf_seq = [ascii_num];
    let ascii_char = std::str::from_utf8(&utf_seq).expect("utf8 sequence");
    return String::from(ascii_char);
}

pub struct Game {
    prev_gamepad: u8,
    select_point: SelectPoint,
    username: String,
    name_selected: bool
}

impl Game {
    pub fn new() -> Self {
        Self {
            username: String::from(""),
            select_point: SelectPoint { x: 0, y: 0 },
            prev_gamepad: 0,
            name_selected: false
        }
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);
        if just_pressed & wasm4::BUTTON_1 != 0 {
            if self.select_point.y == 3 {
                // check if username is 2 characters without a space at the end
                let trimmed_name = self.username.trim();
                if trimmed_name.len() > 1 {
                    self.name_selected = true;
                }
            } else if self.select_point.x == 8 && self.select_point.y == 2  {
              //  self.username.pop();
                if self.username.is_empty() == false && self.username.len() < NAME_LIMIT {
                    let last_char = self.username.chars().nth(self.username.len() - 1).expect("char");
                    // can only come after non-string
                    let space_char: char = " ".chars().nth(0).expect("space char");
                    if last_char != space_char  {
                        self.username.push_str(" ");
                    }
                }
            } else {
                if  self.username.len() < NAME_LIMIT {
                    let pressed_char = get_char_from_xy(self.select_point.x.try_into().unwrap(),
                                                    self.select_point.y.try_into().unwrap());
                    self.username.push_str(&pressed_char);
                }
            }
        }
        
        if just_pressed & wasm4::BUTTON_2 != 0 {
            self.username.pop();
        }

        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.select_point.x -= 1;
            if self.select_point.x < 0 {
                self.select_point.x = 8;
            } 
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.select_point.x += 1;
            if self.select_point.x > 8 {
                self.select_point.x = 0;
            }
        }
        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.select_point.y -= 1;
            if self.select_point.y < 0 {
                self.select_point.y = 2;
            }
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.select_point.y += 1;
            if self.select_point.y > 3 {
                self.select_point.y = 0;
            }
        }

        self.prev_gamepad = gamepad;
    }

    pub fn update(&mut self) {

        if (self.name_selected) {
            let welcome_string = format!("Welcome \n {}!", self.username);
            text(welcome_string, 5, 50);
        } else {
            self.input();

            unsafe { *DRAW_COLORS = 4 }

            //  rect(10, 10, 32, 32);
            if self.select_point.y == 3 {
                text("[    ]",
                     1,
                     25 + 16 * 3);
            } else {
                text("[ ]",
                     16 * self.select_point.x + 1,
                     25 + 16 * self.select_point.y);
            }
            unsafe { *DRAW_COLORS = 2 }

            for yi in 0..3 {
                for xi in 0..9 {
                    let ascii_char = get_char_from_xy(xi, yi);
                    let x_loc: i32 = (9 + xi * 16).into();
                    let y_loc: i32 = (25 + yi * 16).into();
                    if yi == 2 && xi == 8 {
                        //  text(b"\x84", x_loc, y_loc);
                        blit(&SPACE_BAR,
                             x_loc,
                             y_loc,
                             8, 8, BLIT_1BPP);
                        // text(b"\xBBfinish", x_loc, y_loc);
                    } else {
                        text(&ascii_char, x_loc, y_loc);
                    }
                }
            }
            // check if username is 2 characters without a space at the end
            text("DONE", 9, 73);
            unsafe { *DRAW_COLORS = 3 }

            text("What is thy name?", 10, 10);

            text(b"\x80: input ", 10, 90);
            text(b"\x81: delete ", 10, 105);

            text("2-12 characters", 10, 120);
            text(">: ", 10, 135);
            unsafe { *DRAW_COLORS = 4 }

            text(&self.username, 30, 135);
        }
    }

  //  pub fn select_up(&self) {}

  //  pub fn select_down(&self) {}
  //  pub fn select_left(&self) {}
  //  pub fn select_right(&self) {}
}
