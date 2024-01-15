use crate::prelude::*;

use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);

        while mb
            .map
            .tiles
            .iter()
            .filter(|&&tile| tile == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            let flow_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                MAX_FLOWMAP_DISTANCE,
            );

            for (idx, _) in flow_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, &distance)| distance > MAX_FLOWMAP_DISTANCE)
            {
                mb.map.tiles[idx] = TileType::Wall;
            }
        }

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = *start;
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            let directions = &Map::directions();
            let direction = rng
                .random_slice_entry(directions.as_slice())
                .expect("Expected Direction");
            drunkard_pos += *direction;

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;

            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
