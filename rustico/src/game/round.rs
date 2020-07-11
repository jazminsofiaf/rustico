use crate::players::round_type::round_type::RoundType;
use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;

pub struct Round {
    pub round_type : RoundType,
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,

}

impl Round  {
    pub fn new(round_type: RoundType, forbidden_player_id: Option<i32>, game_ended: bool ) -> Round {
        Round {
            round_type,
            forbidden_player_id,
            game_ended,
        }
    }

}
