#![no_std]
#![no_main]
extern crate alloc;

mod audio;
mod game;
mod input;

use core::cell::OnceCell;
use game::Game;

static mut STATE: OnceCell<Game> = OnceCell::new();

fn state() -> &'static mut Game {
    let ptr = &raw mut STATE;
    unsafe { (*ptr).get_mut().unwrap() }
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let game = Game::new(true);
    let ptr = &raw mut STATE;
    unsafe { (*ptr).set(game).ok() };
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    state().update();
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    state().render();
}
