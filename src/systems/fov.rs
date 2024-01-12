use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut views = <(&Point, &mut FieldOfView)>::query();

    for (pos, mut fov) in views.iter_mut(ecs).filter(|(_, fov)| fov.is_dirty) {
        fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
        fov.is_dirty = false;
        println!(
            "calculated fov for {:?}, visible tiles: {}",
            pos,
            fov.visible_tiles.len()
        );
    }
}
