#[cfg(feature = "buddy-alloc")]
mod alloc;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod game;
mod wasm4;
mod select_box;

use game::Game;
use wasm4::*;


lazy_static! {
    static ref MY_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    MY_GAME.lock().expect("game_state").update();
}
