use yew::prelude::*;

use crate::cards;
use crate::board;
use crate::map;

pub struct Game {
  props: Props,
  link: ComponentLink<Self>,
  state: State,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub players: Vec<String>,
  pub max_rounds: usize,
  pub end_game: Callback<()>,
}

pub enum Msg {
  EndTurn,
}

#[derive(Properties, Clone, PartialEq)]
pub struct State {
  pub map: map::Map,
  current_round: usize,
  pub current_player: usize,
  pub players: Vec<PlayerState>,
}

impl State {
  fn new(players: Vec<String>) -> Self {
    let images = crate::map::ImageLoader::load();
    let map = map::Map::new(images, 11, 15);

    let (cx, cy) = map.center();
    // clockwise around center
    let player_offsets:Vec<(i8,i8)> = vec![
      (0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,0),
    ];
    let player_coords = player_offsets.iter().map(|(x, y)| {
      ((x + cx as i8) as u8, (y + cy as i8) as u8)
    }).collect::<Vec<(u8,u8)>>();
    State {
      current_player: 0,
      current_round: 0,
      players: players.iter().enumerate().map(|(i, name)| {
        let (x, y) = player_coords.get(i).unwrap().clone();
        PlayerState {
          x,
          y,
          name: name.clone(),
          cards: map.hand(8),
          food: 10,
          sand: 0,
          sheep: 0,
          stone: 0,
          wood: 0,
        }
      }).collect(),
      map,
    }
  }

  fn current_player(&self) -> &PlayerState {
    self.players.get(self.current_player).unwrap()
  }

  fn end_turn(&mut self) {
    self.current_player += 1;

    if self.current_player >= self.players.len() {
      self.end_round();
    }
  }

  fn end_round(&mut self) {
    self.current_player = 0;
    self.current_round += 1;
  }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PlayerState {
  pub x: u8,
  pub y: u8,
  name: String,
  cards: Vec<map::Tile>,
  food: u8,
  sand: u8,
  sheep: u8,
  stone: u8,
  wood: u8,
}

impl Component for Game {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      state: State::new(props.players.clone()),
      props,
      link,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::EndTurn => {
        self.state.end_turn();
        if self.state.current_round >= self.props.max_rounds {
          self.props.end_game.emit(());
        }
      },
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let player = self.state.current_player();
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
            <table class="table is-hoverable is-fullwidth">
              <tr>
                <td> {"Food"} </td>
                <td> {player.food} </td>
              </tr>
              <tr>
                <td> {"Sand"} </td>
                <td> {player.sand} </td>
              </tr>
              <tr>
                <td> {"Sheep"} </td>
                <td> {player.sheep} </td>
              </tr>
              <tr>
                <td> {"Stone"} </td>
                <td> {player.stone} </td>
              </tr>
              <tr>
                <td> {"Wood"} </td>
                <td> {player.wood} </td>
              </tr>
            </table>
          </div>
          <div class="column">
            <board::Board state={self.state.clone()}/>
          </div>
          <div class="column is-one-quarter has-text-centered">
            <h2 class="subtitle"> {"Round"} </h2>
            <div>
              <span> { self.state.current_round + 1 } {"/"} { self.props.max_rounds } </span>
            </div>
            <h2 class="subtitle"> {"Players"} </h2>
            {
              self.state.players.iter().enumerate().map(|(i, player)| {
                html!{
                  <div class={
                    if i == self.state.current_player {
                      "has-background-info"
                    } else {
                      "has-background-light"
                    }
                  }>
                    <span>
                      {player.name.clone()}
                    </span>
                  </div>
                }
              }).collect::<Vec<Html>>()
            }
          </div>
        </section>
        <section class="columns">
          <div class="column is-one-quarter has-text-centered">
          </div>
          <div class="column">
            <cards::Cards cards={player.cards.clone()}/>
          </div>
          <div class="column is-one-quarter has-text-centered">
            <button class="button is-large" onclick=self.link.callback(|_| Msg::EndTurn)>
            {"End Turn"}
          </button>
          </div>
        </section>
      </div>
    }
  }
}
