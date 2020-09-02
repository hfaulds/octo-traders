// src/lib.rs
#![recursion_limit = "1024"]
mod map;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew::services::{RenderService, Task};

struct Hello {
  canvas: Option<HtmlCanvasElement>,
  ctx: Option<CanvasRenderingContext2d>,
  map: map::Map,
  link: ComponentLink<Self>,
  node_ref: NodeRef,
  render_loop: Option<Box<dyn Task>>,
}

pub enum Msg {
  Render(f64),
}


impl Component for Hello {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      canvas: None,
      ctx: None,
      map: map::Map::random(10, 10),
      link,
      node_ref: NodeRef::default(),
      render_loop: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Render(_) => {
        let ctx = self.ctx.as_ref().expect("Context not initialized!");
        let canvas = self.canvas.as_ref().expect("Cannot get context");

        let map = &self.map;
        for y in 0..map.columns() {
          for x in 0..map.rows() {
            let tile = map.get(x, y);
            // canvas center
            let cx = (canvas.width() / 2) as f64;
            let cy = (canvas.height() / 2) as f64;
            // tile position
            let tx = (x as f64 - (map.rows() as f64 / 2.)) * tile.width;
            let ty = (y as f64 - (map.columns() as f64 / 2.)) * tile.height;
            // tile offset
            let ox = (y % 2) as f64 * (tile.width / 2.);
            let oy = tile.height * 2.;
            ctx
              .draw_image_with_html_image_element_and_dw_and_dh(
                tile.image,
                tx - (tile.width * 1.25) + ox + cx,
                ty - tile.image_width + oy + cy,
                tile.image_width,
                tile.image_height,
              )
              .expect("should draw");
          }
        }

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
      }
    }
    false
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn rendered(&mut self, first_render: bool) {
    let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

    let ctx: CanvasRenderingContext2d = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    self.canvas = Some(canvas);
    self.ctx = Some(ctx);

    if first_render {
      let render_frame = self.link.callback(Msg::Render);
      let handle = RenderService::request_animation_frame(render_frame);
      self.render_loop = Some(Box::new(handle));
    }
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <h1 class="title has-text-centered"> {"Tradathon"} </h1>
        <section class="columns">
          <div class="column is-one-quarter has-text-centered">
            <h2 class="subtitle"> {"Resources"} </h2>
          </div>
          <div class="column">
            <canvas ref={self.node_ref.clone()} height="750px" width="1000px"/>
          </div>
          <div class="column is-one-quarter has-text-centered">
            <h2 class="subtitle"> {"Players"} </h2>
          </div>
        </section>
        <section class="columns">
          <div class="column is-one-quarter has-text-centered">
          </div>
          <div class="column">
            {"ASDF"}
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
  yew::start_app::<Hello>();
}
