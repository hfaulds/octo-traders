use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew::services::{RenderService, Task};

pub struct Board {
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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub state: crate::game::State,
}

impl Component for Board {
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
        ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

        let map = &self.props.state.map;
        for y in 0..map.rows() {
          for x in 0..map.columns() {
            let tile = map.get(x, y);
            // canvas center
            let cx = (canvas.width() / 2) as f64;
            let cy = (canvas.height() / 2) as f64;
            // tile position
            let tx = (x as f64 - (map.columns() as f64 / 2.)) * tile.width;
            let ty = (y as f64 - (map.rows() as f64 / 2.)) * tile.height;
            // tile offset
            let ox = (y % 2) as f64 * (tile.width / 2.);
            let oy = tile.height * -2.;
            ctx
              .draw_image_with_html_image_element_and_dw_and_dh(
                &tile.image,
                tx - (tile.width * 1.25) + ox + cx,
                ty - tile.height + oy + cy,
                tile.image_width,
                tile.image_height,
              )
              .expect("should draw");

            if self.props.state.players.iter().any(|p| p.x == x && p.y == y) {
              if !self.props.state.players.iter().enumerate().any(|(i, p)| p.x == x && p.y == y && i == self.props.state.current_player) {
                ctx.set_global_alpha(0.5);
              }
              ctx
                .draw_image_with_html_image_element_and_dw_and_dh(
                  map.images.get(crate::tiles::UNIT_BOAT),
                  tx - (tile.width * 1.25) + ox + cx,
                  ty - tile.height + oy + cy,
                  tile.image_width,
                  tile.image_height,
                  )
                .expect("should draw");
              ctx.set_global_alpha(1.);
            }
          }
        }

        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
      }
    }
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if props != self.props {
      self.props = props;
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
      <canvas ref={self.node_ref.clone()}
        height={format!("{}px", self.props.state.map.rows() as f64 * 40.)}
        width="1000px"/>
    }
  }
}
