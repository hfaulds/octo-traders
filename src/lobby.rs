use yew::prelude::*;

use crate::game;

pub struct Lobby {
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Clone)]
enum State {
  PreGame{
    players: Vec<String>,
  },
  InGame{
    players: Vec<String>,
  },
}

#[derive(Properties, Clone)]
pub struct Props {
  state: State,
}

impl Default for Props {
  fn default() -> Self {
    Props {
      state: State::PreGame{ players: Vec::new() },
    }
  }
}

pub enum Msg {
  StartGame,
  AddPlayer(String),
  UpdatePlayer(usize, String),
}

impl Component for Lobby {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match &mut self.props.state {
      State::PreGame { players } => {
        match msg {
          Msg::StartGame => {
            self.props.state = State::InGame{ players: players.to_vec() };
          },
          Msg::AddPlayer(name) => {
            players.push(name);
          },
          Msg::UpdatePlayer(i, name) => {
            if name == "" {
              players.remove(i);
            } else {
              players[i] = name;
            }
          },
        };
        true
      },
      _ => false,
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    match &self.props.state {
      State::PreGame { players } => {
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
            <div class="container">
              <div class="field">
                <label class="label">{"Players"}</label>
                {
                  players.iter().enumerate().map(|(i, player)| {
                    html! {
                      <div class="control">
                        <input class="input"
                          type="text"
                          value={player}
                          oninput=self.link.callback(move |e: InputData| Msg::UpdatePlayer(i, e.value))
                        />
                      </div>
                    }
                  }).collect::<Vec<Html>>()
                }
                <div class="control">
                  <input class="input"
                    type="text"
                    placeholder={format!("Player {}", players.len() + 1)}
                    oninput=self.link.callback(|e: InputData| Msg::AddPlayer(e.value))
                    />
                </div>
              </div>
              <button onclick=self.link.callback(move |_| Msg::StartGame) class="button is-large">
                {"Start Game"}
              </button>
            </div>
          </div>
        }
      },
      State::InGame { players } => {
        html! {
          <game::Game players={players} />
        }
      },
    }
  }
}
