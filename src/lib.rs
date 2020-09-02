#![recursion_limit = "1024"]
mod board;
mod cards;
mod game;
mod lobby;
mod map;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<lobby::Lobby>();
}
