use rand::seq::SliceRandom;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

use crate::perlin::PerlinNoise;

#[derive(Clone)]
pub struct Map {
  images: ImageLoader,
  tiles: Vec<Vec<HtmlImageElement>>,
}

impl<'a> Map {
  pub fn new(images: ImageLoader, width: u8, height: u8) -> Map {
    // decide which tiles will be water
    let mut populated_tile_coords = HashMap::new();
    let perlin = PerlinNoise::new();
    for x in 0..width {
      for y in 0..height {
        let p = perlin.get2d([x as f64 * 1.1, y as f64 * 1.1]);
        if x != width/2 &&
           y != height/2 &&
           p > 0.5
        {
          populated_tile_coords.insert((x,y), ());
        }
      }
    }

    // create random list of non-water tiles
    // with guarantee of at least 1 of each resource
    let mut populated_tiles = Vec::new();
    for t in crate::tiles::RESOURCE_TILES.iter().cloned() {
      populated_tiles.push(images.get(t).clone());
    }
    for _ in populated_tiles.len()..populated_tile_coords.len() {
      populated_tiles.push(images.random(&crate::tiles::MAP_TILES).clone());
    }
    populated_tiles.shuffle(&mut rand::thread_rng());

    // create full map with water tiles
    let mut rows = Vec::new();
    for x in 0..width {
      let mut row = Vec::new();
      for y in 0..height {
        if x == width/2 && y == height/2 {
          row.push(images.get(crate::tiles::BUILDING_CASTLE).clone())
        } else if populated_tile_coords.get(&(x,y)).is_some() {
          row.push(populated_tiles.pop().unwrap().clone())
        } else {
          row.push(images.get(crate::tiles::WATER).clone())
        }
      }
      rows.push(row);
    }

    Map {
      images,
      tiles: rows,
    }
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

  pub fn hand(&'a self, num: u8) -> Vec<Tile> {
    let mut hand = Vec::new();
    for i in 0..num {
      let image = if i == (num -1) {
        self.images.get(crate::tiles::BUILDING_CASTLE).clone()
      } else {
        self.images.random(&crate::tiles::RESOURCE_TILES).clone()
      };
      hand.push(
        Tile {
          image,
          image_width: 256.,
          image_height: 256.,
          width: 92.,
          height: 48.,
        }
      )
    }
    hand
  }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

#[derive(Clone)]
pub struct ImageLoader {
  map: HashMap<crate::tiles::TileData, HtmlImageElement>,
}

impl ImageLoader {
  pub fn load() -> ImageLoader {
    let mut map = HashMap::new();

    for tile in crate::tiles::ALL_TILES.iter() {
      let element = HtmlImageElement::new().unwrap();
      element.set_src(&format!("./tiles/{}", tile.image));
      map.insert(tile.clone(), element.clone());
    }

    ImageLoader { map }
  }
}

impl<'a> ImageLoader {
  pub fn get(&'a self, tile: crate::tiles::TileData) -> &'a HtmlImageElement {
    self.map.get(&tile).unwrap()
  }

  pub fn random(&'a self, tiles: &[crate::tiles::TileData]) -> &'a HtmlImageElement {
    let mut images = Vec::new();
    for tile in tiles.iter() {
      images.push(self.map.get(tile).unwrap());
    }
    images.choose(&mut rand::thread_rng()).unwrap()
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
