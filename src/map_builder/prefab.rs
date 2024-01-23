use crate::prelude::*;

pub const FORTRESS: &str = "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
";

const MAX_ATTEMPTS: i32 = 10;
const MIN_DISTANCE_FROM_PLAYER: f32 = 20.0;

pub fn apply_prefab(mb: &mut MapBuilder, prefab: &str, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let flow_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        MAX_FLOWMAP_DISTANCE,
    );

    let (width, height) = prefab_size(prefab);

    let mut attempts = 0;
    while placement.is_none() && attempts < MAX_ATTEMPTS {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - width),
            rng.range(0, SCREEN_HEIGHT - height),
            width,
            height,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = flow_map.map[idx];
            if distance < MAX_FLOWMAP_DISTANCE
                && distance > MIN_DISTANCE_FROM_PLAYER
                && mb.amulet_start != pt
            {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt))
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        place_prefab(placement, mb, FORTRESS);
    }
}

fn prefab_size(prefab: &str) -> (i32, i32) {
    let filled_lines: Vec<&str> = prefab.lines().filter(|line| !line.is_empty()).collect();

    let height: i32 = filled_lines
        .len()
        .try_into()
        .expect("error calculating height");
    let mut width: i32 = 0;
    if let Some(first_line) = filled_lines.first() {
        width = first_line
            .chars()
            .count()
            .try_into()
            .expect("error calculating width");
    }

    (width, height)
}

pub fn place_prefab(placement: Point, mb: &mut MapBuilder, prefab: &str) {
    let (width, height) = prefab_size(prefab);

    let string_vec: Vec<char> = prefab.chars().filter(|&c| c != '\n' && c != '\r').collect();

    let mut i = 0;
    for ty in placement.y..placement.y + height {
        for tx in placement.x..placement.x + width {
            let idx = map_idx(tx, ty);
            let c = string_vec[i];
            match c {
                'M' => {
                    mb.map.tiles[idx] = TileType::Floor;
                    mb.monster_spawns.push(Point::new(tx, ty));
                }
                '-' => mb.map.tiles[idx] = TileType::Floor,
                '#' => mb.map.tiles[idx] = TileType::Wall,
                '@' => mb.player_start = Point::new(tx, ty),
                _ => panic!("Unsupported Prefab Tile \"{}\"", c),
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::empty::EmptyArchitect;

    use super::*;
    fn map_builder() -> MapBuilder {
        MapBuilder::new()
    }

    fn count_walls(map: Map) -> usize {
        map.tiles
            .iter()
            .filter(|&&tile| tile == TileType::Wall)
            .count()
    }

    #[test]
    fn test_place_fortress() {
        let mut rng = RandomNumberGenerator::new();

        let mut mb = EmptyArchitect {}.build(&mut rng);

        apply_prefab(&mut mb, FORTRESS, &mut rng);

        assert_eq!(count_walls(mb.map), 32)
    }

    #[test]
    fn test_fortress_size() {
        let (width, height) = prefab_size(FORTRESS);
        assert_eq!(width, 12);
        assert_eq!(height, 11);
    }
}
