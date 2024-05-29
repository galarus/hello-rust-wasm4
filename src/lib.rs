#[cfg(feature = "buddy-alloc")]
mod alloc;
use std::sync::Mutex;
use lazy_static::lazy_static;
mod game;
mod wasm4;
use game::Game;
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

lazy_static! {
        static ref MY_GAME: Mutex<Game> = Mutex::new(Game::new());
}

//static selectSwitch: AtomicBool = AtomicBool::new(false);

pub static username: Mutex<String> = Mutex::new(String::new());   

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    MY_GAME.lock().expect("game_state").update();
    let gamepad = unsafe { *GAMEPAD1 };
/*    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
        let prevName = &*username.lock().unwrap().clone();
        let newName = format!("{}{}", prevName, "A");
        *username.lock().unwrap() = String::from(newName);
        *selectSwitch.get_mut() = true;
    } else {
        *selectSwitch.get_mut() = false;
    }
*/

}
