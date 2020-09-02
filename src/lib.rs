#![recursion_limit = "1024"]
mod board;
mod cards;
mod game;
mod lobby;
mod map;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Root {
  link: ComponentLink<Self>,
  state: State,
}

#[derive(Clone)]
enum State {
  PreGame,
  InGame{ players: Vec<String> },
}

pub enum Msg {
  StartGame{ players: Vec<String> },
  EndGame,
}

impl Component for Root {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      state: State::PreGame,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::StartGame { players } => {
        self.state = State::InGame { players };
        true
      },
      Msg::EndGame => {
        self.state = State::PreGame;
        true
      },
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    match &self.state {
      State::PreGame => {
        html! {
          <lobby::Lobby start_game=self.link.callback(move |players| Msg::StartGame{ players })/>
        }
      },
      State::InGame { players } => {
        html! {
          <game::Game
            players={players}
            max_rounds={2}
            end_game=self.link.callback(move |players| Msg::EndGame)/>
        }
      },
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<Root>();
}
