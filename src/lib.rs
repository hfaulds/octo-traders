// src/lib.rs
#![recursion_limit = "1024"]
mod map;
mod board;
mod cards;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Game {
  map: map::Map,
  cards: Vec<map::Tile>
}

pub enum Msg {
  Render(f64),
}

impl Component for Game {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let map = map::Map::new(9, 9);
    let cards = map.random(8);
    Self {
      map,
      cards,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <h1 class="title has-text-centered"> {"Octo Traders"} </h1>
        <section class="columns">
          <div class="column is-one-quarter has-text-centered">
            <h2 class="subtitle"> {"Resources"} </h2>
            <div> <span> {"Food"} </span> </div>
            <div> <span> {"Sand"} </span> </div>
            <div> <span> {"Sheep"} </span> </div>
            <div> <span> {"Stone"} </span> </div>
            <div> <span> {"Wood"} </span> </div>
          </div>
          <div class="column">
            <board::Board map={self.map.clone()}/>
          </div>
          <div class="column is-one-quarter has-text-centered">
            <h2 class="subtitle"> {"Players"} </h2>
            <div> <span> {"Hayden"} </span> </div>
            <div> <span> {"Bob"} </span> </div>
            <div> <span> {"Alice"} </span> </div>
          </div>
        </section>
        <section class="columns">
          <div class="column is-one-quarter has-text-centered">
          </div>
          <div class="column">
            <cards::Cards cards={self.cards.clone()}/>
          </div>
          <div class="column is-one-quarter has-text-centered">
            <button class="button is-large"> {"End Turn"} </button>
          </div>
        </section>
      </div>
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<Game>();
}
