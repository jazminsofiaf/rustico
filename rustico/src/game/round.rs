

pub struct Round {
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,
}

impl Round  {
    pub fn new(forbidden_player_id: Option<i32>, game_ended: bool, ) -> Round {
        Round {
            forbidden_player_id,
            game_ended,
        }
    }
}
