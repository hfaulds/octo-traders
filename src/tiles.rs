#[derive(Clone,Eq,PartialEq,Hash)]
pub struct TileData {
  pub image: &'static str,
  pub resource: ResourceType,
}

#[derive(Clone,Eq,PartialEq,Hash)]
pub enum ResourceType {
  None,
  Sand,
  Sheep,
  Stone,
  Wood,
}

pub const RESOURCE_TILES: [TileData; 4] = [
  BUILDING_MINE,
  GRASS_FOREST,
  SAND_ROCKS,
  BUILDING_SHEEP,
];

pub const MAP_TILES: [TileData; 24] = [
  BUILDING_MINE,
  WATER,
  BUILDING_WATER,
  GRASS_HILL,
  GRASS_FOREST,
  DIRT_LUMBER,
  STONE,
  DIRT,
  BUILDING_MARKET,
  BUILDING_HOUSE,
  BUILDING_TOWER,
  BUILDING_SMELTER,
  STONE_HILL,
  BUILDING_CABIN,
  BUILDING_WALL,
  BUILDING_DOCK,
  STONE_MOUNTAIN,
  BUILDING_VILLAGE,
  BUILDING_FARM,
  BUILDING_MILL,
  GRASS,
  BUILDING_SHEEP,
  SAND_ROCKS,
  WATER_ROCKS,
];

pub const ALL_TILES: [TileData; 25] = [
  BUILDING_MINE,
  WATER,
  BUILDING_WATER,
  GRASS_HILL,
  GRASS_FOREST,
  DIRT_LUMBER,
  STONE,
  DIRT,
  BUILDING_MARKET,
  BUILDING_HOUSE,
  BUILDING_TOWER,
  BUILDING_SMELTER,
  STONE_HILL,
  BUILDING_CABIN,
  BUILDING_WALL,
  BUILDING_DOCK,
  STONE_MOUNTAIN,
  BUILDING_VILLAGE,
  BUILDING_FARM,
  BUILDING_MILL,
  GRASS,
  BUILDING_SHEEP,
  SAND_ROCKS,
  BUILDING_CASTLE,
  WATER_ROCKS,
];

pub const BUILDING_MINE: TileData = TileData {
  image: "building_mine_S.png",
  resource: ResourceType::Stone,
};
pub const WATER: TileData = TileData {
  image: "water_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_WATER: TileData = TileData {
  image: "building_water_S.png",
  resource: ResourceType::None,
};
pub const GRASS_HILL: TileData = TileData {
  image: "grass_hill_S.png",
  resource: ResourceType::None,
};
pub const GRASS_FOREST: TileData = TileData {
  image: "grass_forest_S.png",
  resource: ResourceType::Wood,
};
pub const DIRT_LUMBER: TileData = TileData {
  image: "dirt_lumber_S.png",
  resource: ResourceType::None,
};
pub const STONE: TileData = TileData {
  image: "stone_S.png",
  resource: ResourceType::None,
};
pub const DIRT: TileData = TileData {
  image: "dirt_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_MARKET: TileData = TileData {
  image: "building_market_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_HOUSE: TileData = TileData {
  image: "building_house_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_TOWER: TileData = TileData {
  image: "building_tower_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_SMELTER: TileData = TileData {
  image: "building_smelter_S.png",
  resource: ResourceType::None,
};
pub const STONE_HILL: TileData = TileData {
  image: "stone_hill_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_CABIN: TileData = TileData {
  image: "building_cabin_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_WALL: TileData = TileData {
  image: "building_wall_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_DOCK: TileData = TileData {
  image: "building_dock_S.png",
  resource: ResourceType::None,
};
pub const STONE_MOUNTAIN: TileData = TileData {
  image: "stone_mountain_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_VILLAGE: TileData = TileData {
  image: "building_village_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_FARM: TileData = TileData {
  image: "building_farm_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_MILL: TileData = TileData {
  image: "building_mill_S.png",
  resource: ResourceType::None,
};
pub const GRASS: TileData = TileData {
  image: "grass_S.png",
  resource: ResourceType::None,
};
pub const BUILDING_SHEEP: TileData = TileData {
  image: "building_sheep_S.png",
  resource: ResourceType::Sheep,
};
pub const SAND_ROCKS: TileData = TileData {
  image: "sand_rocks_S.png",
  resource: ResourceType::Sand,
};
pub const BUILDING_CASTLE: TileData = TileData {
  image: "building_castle_S.png",
  resource: ResourceType::None,
};
pub const WATER_ROCKS: TileData = TileData {
  image: "water_rocks_S.png",
  resource: ResourceType::None,
};
