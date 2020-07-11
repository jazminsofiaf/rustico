use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;
use crate::game::round::Round;
use crate::game::normal_round::NormalRound;

const ONE_POINT: i32 = 1;
const FIVE_POINTS: i32 = 5;

pub struct RusticRound {
    pub len: i32,
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,

}

impl  RusticRound {
    pub fn new(len: i32, forbidden_player_id: Option<i32>, game_ended: bool) -> RusticRound {
        RusticRound {
            len,
            forbidden_player_id,
            game_ended,
        }
    }
}

impl Round for RusticRound {

    fn get_next_rustic_round(&self, number_of_players: i32, last_player_id: i32) -> RusticRound{
        let round_len = number_of_players - 1;
        RusticRound::new( round_len, Some(last_player_id), false)
    }

    fn get_next_normal_round(&self, number_of_players: i32, last_player_id: i32) -> NormalRound {
        let round_len = number_of_players - 1;
        NormalRound::new( round_len, Some(last_player_id), false)
    }


    fn compute_score(&self, hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
        players = self.base.compute_score(hand, players);

        players[hand.first().unwrap().player_id as usize].win_points(ONE_POINT);
        players[hand.last().unwrap().player_id as usize].lose_points(FIVE_POINTS);

        return players;
    }


}

