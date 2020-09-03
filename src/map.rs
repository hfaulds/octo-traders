use rand::seq::SliceRandom;
use std::collections::HashMap;
use web_sys::HtmlImageElement;
use log::info;

use crate::perlin::PerlinNoise;

#[derive(Clone,PartialEq)]
pub struct Map {
  tiles: Vec<Vec<HtmlImageElement>>,
}

impl<'a> Map {
  pub fn new(width: u8, height: u8) -> Map {
    let tiles = Tiles::load();
    let mut rows = Vec::new();
    let perlin = PerlinNoise::new();

    for x in 0..width {
      let mut row = Vec::new();
      for y in 0..height {
        let p = perlin.get2d([x as f64 * 1.1, y as f64 * 1.1]);
        info!("({} {}) {}", x, y, p);
        if p > 0.5 {
          row.push(tiles.random().clone())
        } else {
          row.push(tiles.get("water_S.png").clone())
        }
      }
      rows.push(row);
    }
    Map { tiles: rows }
  }

  pub fn rows(&self) -> u8 {
    self.tiles.get(0).unwrap().len() as u8
  }

  pub fn columns(&self) -> u8 {
    self.tiles.len() as u8
  }

  pub fn get(&self, x: u8, y: u8) -> Tile {
    Tile {
      image: self.tiles.get(x as usize).unwrap().get(y as usize).unwrap().clone(),
      image_width: 192.,
      image_height: 192.,
      width: 69.,
      height: 36.,
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
  map: HashMap<String, HtmlImageElement>,
  vec: Vec<HtmlImageElement>,
}

impl Tiles {
  pub fn load() -> Tiles {
    let mut vec: Vec<HtmlImageElement> = Vec::new();
    let mut map: HashMap<String, HtmlImageElement> = HashMap::new();

    for tile in crate::tiles::ALL_TILES.iter() {
      let element = HtmlImageElement::new().unwrap();
      element.set_src(&format!("./tiles/{}", tile.image));
      map.insert(String::from(tile.image), element.clone());
      if tile.image != "water_S.png" {
        vec.push(element);
      }
    }

    Tiles { map, vec }
  }
}

impl<'a> Tiles {
  pub fn get(&'a self, name: & str) -> &'a HtmlImageElement {
    self.map.get(name).unwrap()
  }

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
