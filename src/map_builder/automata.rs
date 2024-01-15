use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();
        self.random_noise_map(rng, &mut mb.map);

        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }

        let start = self.find_start(&mb.map);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = self.spawn_monsters(&start, &mut mb.map, rng);
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        for tile in map.tiles.iter_mut() {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *tile = TileType::Floor;
            } else {
                *tile = TileType::Wall;
            }
        }
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;

        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .unwrap()
            .0;

        map.index_to_point2d(closest_point)
    }

    fn spawn_monsters(
        &self,
        start: &Point,
        map: &mut Map,
        rng: &mut RandomNumberGenerator,
    ) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        const MIN_SPAWN_DISTANCE: f32 = 10.0;
        let mut spawnable_tiles: Vec<Point> = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, tile)| {
                **tile == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, map.index_to_point2d(*idx))
                        > MIN_SPAWN_DISTANCE
            })
            .map(|(idx, _)| map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();

        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }

        spawns
    }
}
