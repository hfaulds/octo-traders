use rand::seq::SliceRandom;
use web_sys::HtmlImageElement;

#[derive(Clone,PartialEq)]
pub struct Map {
  tiles: Vec<Vec<HtmlImageElement>>,
}

impl<'a> Map {
  pub fn new(x: u8, y: u8) -> Map {
    let tiles = Tiles::load();
    let mut rows = Vec::new();
    for _ in 0..x {
      let mut row = Vec::new();
      for _ in 0..y {
        row.push(tiles.random().clone())
      }
      rows.push(row);
    }
    Map { tiles: rows }
  }

  pub fn rows(&self) -> u8 {
    self.tiles.len() as u8
  }

  pub fn columns(&self) -> u8 {
    self.tiles.get(0).unwrap().len() as u8
  }

  pub fn get(&self, x: u8, y: u8) -> Tile {
    Tile {
      image: self.tiles.get(x as usize).unwrap().get(y as usize).unwrap().clone(),
      image_width: 256.,
      image_height: 256.,
      width: 92.,
      height: 48.,
    }
  }

  pub fn random(&'a self, num: u8) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let rng = &mut rand::thread_rng();
    for _ in 0..num {
      let image = self.tiles.choose(rng).unwrap()
        .choose(rng).unwrap().clone();
      tiles.push(
        Tile {
          image,
          image_width: 256.,
          image_height: 256.,
          width: 92.,
          height: 48.,
        }
      )
    }
    tiles
  }
}

pub struct Tiles {
  vec: Vec<HtmlImageElement>,
}

impl Tiles {
  pub fn load() -> Tiles {
    let images = vec![
      "building_mine_S.png",
      "water_S.png",
      "building_water_S.png",
      "sand_S.png",
      //"unit_houseLarge_S.png",
      //"path_end_S.png",
      "grass_hill_S.png",
      "grass_forest_S.png",
      "dirt_lumber_S.png",
      "stone_S.png",
      "stone_rocks_S.png",
      //"river_cornerSharp_S.png",
      //"path_straight_S.png",
      //"river_straight_S.png",
      //"path_cornerSharp_S.png",
      //"river_intersectionH_S.png",
      //"path_intersectionH_S.png",
      "dirt_S.png",
      //"unit_tower_S.png",
      "building_market_S.png",
      "building_house_S.png",
      "building_tower_S.png",
      "building_smelter_S.png",
      "stone_hill_S.png",
      //"unit_house_S.png",
      //"unit_mill_S.png",
      //"path_corner_S.png",
      "building_cabin_S.png",
      "building_wall_S.png",
      //"path_start_S.png",
      "building_dock_S.png",
      //"river_crossing_S.png",
      "stone_mountain_S.png",
      "building_village_S.png",
      //"path_crossing_S.png",
      "building_farm_S.png",
      //"path_intersectionB_S.png",
      //"river_intersectionB_S.png",
      //"river_intersectionF_S.png",
      //"path_intersectionF_S.png",
      //"unit_wallTower_S.png",
      "building_mill_S.png",
      //"river_intersectionD_S.png",
      //"path_intersectionD_S.png",
      "grass_S.png",
      //"river_intersectionC_S.png",
      //"path_intersectionC_S.png",
      //"river_intersectionA_S.png",
      //"path_intersectionA_S.png",
      "building_sheep_S.png",
      "sand_rocks_S.png",
      "building_castle_S.png",
      //"unit_boat_S.png",
      //"path_intersectionE_S.png",
      "water_island_S.png",
      //"river_intersectionE_S.png",
      //"river_end_S.png",
      //"unit_tree_S.png",
      "water_rocks_S.png",
      //"river_corner_S.png",
      //"path_intersectionG_S.png",
      //"river_intersectionG_S.png",
      //"river_start_S.png",
      ];
    let mut vec: Vec<HtmlImageElement> = Vec::new();

    for image in images {
      let element = HtmlImageElement::new().unwrap();
      element.set_src(&format!("./tiles/{}", image));
      vec.push(element);
    }

    Tiles { vec }
  }
}

impl<'a> Tiles {
  pub fn random(&'a self) -> &'a HtmlImageElement {
    self.vec.choose(&mut rand::thread_rng()).unwrap()
  }
}

#[derive(Clone,PartialEq)]
pub struct Tile {
  pub image: HtmlImageElement,
  pub image_width: f64,
  pub image_height: f64,
  pub width: f64,
  pub height: f64,
}
