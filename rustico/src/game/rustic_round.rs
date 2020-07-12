use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;
use crate::game::round::Round;
use crate::game::normal_round::NormalRound;
use crate::players::round_type::round_type::RoundType;


const ONE_POINT: i32 = 1;
const FIVE_POINTS: i32 = 5;

pub struct RusticRound {
    name: RoundType,
    forbidden_player_id: Option<i32>,
    game_ended: bool,

}

impl  RusticRound {
    pub fn new(forbidden_player_id: Option<i32>, game_ended: bool) -> RusticRound {
        RusticRound {
            name: RoundType::RUSTIC,
            forbidden_player_id,
            game_ended,
        }
    }
}

impl Round for RusticRound {

    fn get_name(&self) -> RoundType {
        return self.name;
    }

    fn get_forbidden_player_id(&self) -> Option<i32> {
        return self.forbidden_player_id;
    }

    fn is_game_ended(&self) -> bool {
        return self.game_ended;
    }

    fn get_next_rustic_round(&self,  last_player_id: i32) -> RusticRound{
        RusticRound::new( Some(last_player_id), false)
    }

    fn get_next_normal_round(&self,  last_player_id: i32) -> NormalRound {
        NormalRound::new( Some(last_player_id), false)
    }


    fn compute_score(&self, hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
        players = self.compute_score_default(hand.as_ref(), players);

        players[hand.first().unwrap().player_id as usize].win_points(ONE_POINT);
        players[hand.last().unwrap().player_id as usize].lose_points(FIVE_POINTS);

        return players;
    }


}

