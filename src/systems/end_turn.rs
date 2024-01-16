use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver => *turn_state,
        TurnState::Victory => *turn_state,
        TurnState::NextLevel => *turn_state,
    };

    let amulet_pos = amulet.iter(ecs).next();

    for (hp, pos) in player.iter(ecs) {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if let Some(amulet_pos) = amulet_pos {
            if pos == amulet_pos {
                new_state = TurnState::Victory;
            }
        }
        if let Some(TileType::Exit) = map.try_tile(*pos) {
            new_state = TurnState::NextLevel;
        }
    }

    *turn_state = new_state;
}
