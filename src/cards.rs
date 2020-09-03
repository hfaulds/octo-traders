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

  selection: Selection,
}

pub enum Msg {
  Render(f64),
  MouseMove(f64, f64),
  MouseUp(f64, f64),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<map::Tile>
}

enum Selection {
  None,
  Hover(usize),
  Selected(usize),
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
      selection: Selection::None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    let canvas = self.canvas.as_ref().expect("Cannot get context");
    match msg {
      Msg::MouseUp(mx, my) => {
        self.selection = Selection::None;
        if let Some(x) = collide(mx, my, &self.props.cards, canvas) {
            self.selection = Selection::Selected(x);
        }
      },
      Msg::MouseMove(mx, my) => {
        if let Selection::Selected(_) = self.selection {
          return false;
        }
        self.selection = Selection::None;
        if let Some(x) = collide(mx, my, &self.props.cards, canvas) {
            self.selection = Selection::Hover(x);
        }
      },
      Msg::Render(_) => {
        let ctx = self.ctx.as_ref().expect("Context not initialized!");
        ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

        for (x, card) in self.props.cards.iter().enumerate() {
          let mut image_width = card.image_width;
          let mut image_height = card.image_height;
          if let Selection::Hover(hx) = self.selection {
            if hx == x {
              image_width = image_width * 1.1;
              image_height = image_height * 1.1;
            }
          }
          if let Selection::Selected(hx) = self.selection {
            if hx == x {
              image_width = image_width * 1.1;
              image_height = image_height * 1.1;
            } else {
              ctx.set_global_alpha(0.5);
            }
          }
          let (tx, ty) =  pos(x, self.props.cards.len(), card, image_width, image_height, canvas);
          ctx
            .draw_image_with_html_image_element_and_dw_and_dh(
              &card.image,
              tx,
              ty,
              image_width,
              image_height,
            )
            .expect("should draw");
          ctx.set_global_alpha(1.);
        }

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
      },
    }
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.props != props {
      self.props = props
    }
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
      <canvas
        ref={self.node_ref.clone()}
        height="150px"
        width="1000px"
        onmousemove=self.link.callback(|e: MouseEvent| Msg::MouseMove(e.client_x() as f64, e.client_y() as f64))
        onmouseup=self.link.callback(|e: MouseEvent| Msg::MouseUp(e.client_x() as f64, e.client_y() as f64))
        />
    }
  }
}

fn pos(x: usize, card_count: usize, card: &map::Tile, image_width: f64, image_height: f64, canvas: &HtmlCanvasElement) -> (f64, f64) {
  // canvas center
  let cx = (canvas.width() as f64 - image_width) / 2.;
  let cy = (canvas.height() as f64 - image_height) / 2.;
  // card position
  let tx = (x as f64 - (card_count as f64 / 2.) + 0.5) * (card.width + 10.) + cx;
  let ty = cy - 60.;
  return (tx, ty);
}

fn collide(mx: f64, my: f64, cards: &Vec<map::Tile>, canvas: &HtmlCanvasElement) -> Option<usize> {
  let bb = canvas.get_bounding_client_rect();
  let mx = mx - bb.x();
  let my = my - bb.y();

  for (i, card) in cards.iter().enumerate() {
    let (tx, ty) =  pos(i, cards.len(), card, card.image_width, card.image_height, canvas);

    let tx = tx + (card.image_width - card.width) / 2.0;
    let ty = ty + (card.image_height/2.0) + 20.;

    if mx > tx && mx < tx + card.width &&
      my > ty && my < ty + card.height
      {
        return Some(i)
      }
  }
  None
}
