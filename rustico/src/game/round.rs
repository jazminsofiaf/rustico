

pub struct Round {
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,
    pub player_turn: Option<i32>,
}

impl Round  {
    pub fn new(forbidden_player_id: Option<i32>, game_ended: bool, player_turn: Option<i32>, ) -> Round {
        Round {
            forbidden_player_id,
            game_ended,
            player_turn,
        }
    }
}
