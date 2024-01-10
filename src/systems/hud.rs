use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());

    let player_health = health_query.iter(ecs).nth(0).unwrap(); // Don't like this.

    let mut draw_batch = DrawBatch::new();

    draw_batch.target(LAYER_HUD);

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(RED, BLACK),
    );

    draw_batch.submit(10000).expect("HUD draw batch error");
}
