use yew::prelude::*;

pub struct Lobby {
  players: Vec<String>,
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub start_game: Callback<Vec<String>>,
}

pub enum Msg {
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
      players: Vec::new(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddPlayer(name) => {
        self.players.push(name);
      },
      Msg::UpdatePlayer(i, name) => {
        if name == "" {
          self.players.remove(i);
        } else {
          self.players[i] = name;
        }
      },
    };
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let players = self.players.clone();
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
            {
              if players.len() < 6 {
                html! {
                  <div class="control">
                    <input class="input"
                      type="text"
                      placeholder={format!("Player {}", players.len() + 1)}
                      oninput=self.link.callback(|e: InputData| Msg::AddPlayer(e.value))
                      />
                  </div>
                }
              } else {
                html! {}
              }
            }
          </div>
          <button onclick=self.props.start_game.reform(move |_| players.clone()) class="button is-large">
            {"Start Game"}
          </button>
        </div>
      </div>
    }
  }
}
