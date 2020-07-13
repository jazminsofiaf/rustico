use crate::players::round_type::round_type::RoundType;
use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;
use rand::Rng;
use crate::players::player_game::PlayerGame;
use crate::game::normal_round::NormalRound;
use crate::game::rustic_round::RusticRound;
use colored::*;


const TEN_POINTS: i32 = 10;


pub fn get_random_type_round() -> Box<dyn Round> {
    let mut rng = rand::thread_rng();
    let round_type: RoundType = rng.gen();
    return match round_type {
        RoundType::NORMAL => {
            Box::new(NormalRound::new(Option::None, false))
        }
        RoundType::RUSTIC => {
            Box::new(RusticRound::new(Option::None, false))
        }
    };
}


pub trait Round: Send + Sync {
    fn get_name(&self) -> RoundType;

    fn get_forbidden_player_id(&self) -> Option<i32>;
    fn is_game_ended(&self) -> bool;

    fn get_next_round(&self, last_player_id: i32) -> Box<dyn Round> {
        let mut rng = rand::thread_rng();
        let round_type: RoundType = rng.gen();
        return match round_type {
            RoundType::NORMAL => {
                Box::new(self.get_next_normal_round(last_player_id))
            }
            RoundType::RUSTIC => {
                Box::new(self.get_next_rustic_round(last_player_id))
            }
        };
    }

    fn get_next_rustic_round(&self, last_player_id: i32) -> RusticRound;

    fn get_next_normal_round(&self, last_player_id: i32) -> NormalRound;


    fn wait_turn(&self, _player: &PlayerGame) {
        /* default: do nothing */
    }

    fn should_skip_this_round(&self, player: &PlayerGame) -> bool {
        match self.get_forbidden_player_id() {
            Some(forbidden_id) if forbidden_id == player.get_id() => {
                return true;
            }
            _ => {}
        }
        return false;
    }

    fn end_turn(&self, _player: &PlayerGame) {
        /* default: do nothing */
    }


    fn compute_score_default(&self, hand: &Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
        let winner_response = hand.iter()
            .max_by(|one, other| one.card.cmp(&other.card))
            .unwrap();
        let draw = hand.iter()
            .filter(|response| !(response.card < winner_response.card)).collect::<Vec<_>>();

        let points = TEN_POINTS / draw.len() as i32;

        for winner_card in draw {
            println!("{}", format!("Sending {} points to player {} who sent the highest card : {}",
                                   points, winner_card.player_id,
                                   winner_card.card).black().on_white().italic());
            players[winner_card.player_id as usize].win_points(points);
        }
        return players;
    }

    fn compute_score(&self, hand: Vec<PlayerCard>, players: Vec<Player>) -> Vec<Player> {
        return self.compute_score_default(hand.as_ref(), players);
    }
}
