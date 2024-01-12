use legion::world::SubWorld;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ENTITIES);

    let offset = camera.offset();

    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(*pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Entity Draw Batch Error");
}
