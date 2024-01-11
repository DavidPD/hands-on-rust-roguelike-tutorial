pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health::new(10),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Name(name),
        Health::new(hp),
        Enemy,
        pos,
        ChasingPlayer,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

pub fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

pub fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
