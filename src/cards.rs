use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew::services::{RenderService, Task};

use crate::map;

pub struct Cards {
  canvas: Option<HtmlCanvasElement>,
  ctx: Option<CanvasRenderingContext2d>,
  props: Props,
  link: ComponentLink<Self>,
  node_ref: NodeRef,
  render_loop: Option<Box<dyn Task>>,
}

pub enum Msg {
  Render(f64),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub cards: Vec<map::Tile>
}

impl Component for Cards {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      canvas: None,
      ctx: None,
      props,
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

        for (x, card) in self.props.cards.iter().enumerate() {
          // canvas center
          let cx = (canvas.width() as f64 - card.image_width) / 2.;
          // card position
          let tx = (x as f64 - (self.props.cards.len() as f64 / 2.) + 0.5) * (card.width + 10.);
          ctx
            .draw_image_with_html_image_element_and_dw_and_dh(
              &card.image,
              cx + tx,
              (card.image_height * -0.5) + card.height,
              card.image_width,
              card.image_height,
            )
            .expect("should draw");
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
      <canvas ref={self.node_ref.clone()} height="150px" width="1000px"/>
    }
  }
}
