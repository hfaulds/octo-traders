// src/lib.rs
#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlImageElement;
use yew::prelude::*;
use yew::services::{RenderService, Task};
use std::collections::HashMap;

struct Hello {
  canvas: Option<HtmlCanvasElement>,
  ctx: Option<CanvasRenderingContext2d>,
  images: HashMap<String, HtmlImageElement>,
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
      images: load_images(),
      link,
      node_ref: NodeRef::default(),
      render_loop: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Render(_) => {
        let ctx = self.ctx.as_ref().expect("Context not initialized!");

        for y in 0..9 {
          for x in 0..9 {
            ctx
              .draw_image_with_html_image_element_and_dw_and_dh(
                &self.images.get("grass_S.png").expect(""),
                x as f64 * 92. + (y % 2) as f64 * 46.,
                y as f64 * 46.,
                250.,
                250.,
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
      // The callback to request animation frame is passed a time value which can be used for
      // rendering motion independent of the framerate which may vary.
      let render_frame = self.link.callback(Msg::Render);
      let handle = RenderService::request_animation_frame(render_frame);

      // A reference to the handle must be stored, otherwise it is dropped and the render won't
      // occur.
      self.render_loop = Some(Box::new(handle));
    }
  }

  fn view(&self) -> Html {
    html! {
      <section class="section">
        <div class="container">
          <canvas ref={self.node_ref.clone()} height="1000px" width="1000px"/>
        </div>
      </section>
    }
  }
}

fn load_images() -> HashMap<String, HtmlImageElement> {
  let images = vec![
    "building_mine_S.png",
    "water_S.png",
    "building_water_S.png",
    "sand_S.png",
    "unit_houseLarge_S.png",
    "path_end_S.png",
    "grass_hill_S.png",
    "grass_forest_S.png",
    "dirt_lumber_S.png",
    "stone_S.png",
    "stone_rocks_S.png",
    "river_cornerSharp_S.png",
    "path_straight_S.png",
    "river_straight_S.png",
    "path_cornerSharp_S.png",
    "river_intersectionH_S.png",
    "path_intersectionH_S.png",
    "dirt_S.png",
    "unit_tower_S.png",
    "building_market_S.png",
    "building_house_S.png",
    "building_tower_S.png",
    "building_smelter_S.png",
    "stone_hill_S.png",
    "unit_house_S.png",
    "unit_mill_S.png",
    "path_corner_S.png",
    "building_cabin_S.png",
    "building_wall_S.png",
    "path_start_S.png",
    "building_dock_S.png",
    "river_crossing_S.png",
    "stone_mountain_S.png",
    "building_village_S.png",
    "path_crossing_S.png",
    "building_farm_S.png",
    "path_intersectionB_S.png",
    "river_intersectionB_S.png",
    "river_intersectionF_S.png",
    "path_intersectionF_S.png",
    "unit_wallTower_S.png",
    "building_mill_S.png",
    "river_intersectionD_S.png",
    "path_intersectionD_S.png",
    "grass_S.png",
    "river_intersectionC_S.png",
    "path_intersectionC_S.png",
    "river_intersectionA_S.png",
    "path_intersectionA_S.png",
    "building_sheep_S.png",
    "sand_rocks_S.png",
    "building_castle_S.png",
    "unit_boat_S.png",
    "path_intersectionE_S.png",
    "water_island_S.png",
    "river_intersectionE_S.png",
    "river_end_S.png",
    "unit_tree_S.png",
    "water_rocks_S.png",
    "river_corner_S.png",
    "path_intersectionG_S.png",
    "river_intersectionG_S.png",
    "river_start_S.png",
  ];
  let mut elements: HashMap<String, HtmlImageElement> = HashMap::new();
  for image in images {
    let element = HtmlImageElement::new().unwrap();
    element.set_src(&format!("./tiles/{}", image));
    elements.insert(String::from(image), element);
  }
  elements
}

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<Hello>();
}
