use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver => *turn_state,
    };

    for hp in player_hp.iter(ecs) {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    }

    *turn_state = new_state;
}
