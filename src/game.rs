use yew::prelude::*;

use crate::board;
use crate::cards;
use crate::map;

pub struct Game {
  props: Props,
  map: map::Map,
  cards: Vec<map::Tile>
}

#[derive(Properties, Clone)]
pub struct Props {
  pub players: Vec<String>,
}

impl Component for Game {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    let map = map::Map::new(9, 9);
    let cards = map.random(8);
    Self {
      props,
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
        <section class="hero is-info">
          <div class="hero-body">
            <div class="container">
              <h1 class="title">
                {"Octo Traders"}
              </h1>
            </div>
          </div>
        </section>
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
            {
              self.props.players.iter().map(|(player)| {
                html!{
                  <div><span> {player} </span></div>
                }
              }).collect::<Vec<Html>>()
            }
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
