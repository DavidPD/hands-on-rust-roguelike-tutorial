use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());

    let player_health = health_query.iter(ecs).next().unwrap();

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

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(&entity, _)| entity)
        .next()
        .unwrap();

    let mut item_query = <(&Item, &Name, &Carried)>::query();

    let mut y = 3;
    for (_, name, _) in item_query
        .iter(ecs)
        .filter(|(_, _, &Carried(carried_by))| carried_by == player)
    {
        draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
        y += 1;
    }

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items Carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("HUD draw batch error");
}
