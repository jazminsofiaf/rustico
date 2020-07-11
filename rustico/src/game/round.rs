use crate::players::round_type::round_type::RoundType;
use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;
use rand::Rng;
use crate::players::player_game::PlayerGame;


const TEN_POINTS: i32 = 10;
const ONE_POINT: i32 = 1;
const FIVE_POINTS: i32 = 5;

pub struct Round {
    pub round_type : RoundType,
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,

}


impl Round  {
    pub fn new(forbidden_player_id: Option<i32>, game_ended: bool ) -> Round {
        Round {
            round_type: Round::get_round_type(),
            forbidden_player_id,
            game_ended,
        }
    }

    pub fn get_round_type() -> RoundType {
        let mut rng = rand::thread_rng();
        let round_type: RoundType = rng.gen();
        return round_type;
    }

    pub fn wait_turn(&self, player: &PlayerGame){
        match self.round_type {
            RoundType::NORMAL => {
                player.wait_my_turn();
            }
            _ => {}
        }
    }

    pub fn should_skip_this_round(&self, player: &PlayerGame) -> bool{
        match self.forbidden_player_id {
            Some(forbidden_id) if forbidden_id == player.get_id() =>   {
                match self.round_type {
                    RoundType::NORMAL => {
                        player.notify_next_player_turn();
                    }
                    _ => {}
                }
                return true;
            }
            _ => {}
        }
        return false;

    }

    pub fn end_turn(&self, player: &PlayerGame){
        match self.round_type {
            RoundType::NORMAL => {
                player.notify_next_player_turn();
            }

            _ => {}
        }
    }



    pub fn get_next_round(&self, last_player_id: i32)-> Round{
        return match self.round_type {
            RoundType::RUSTIC => {
                Round::new(  Some(last_player_id), false)
            }
            RoundType::NORMAL => {
                Round::new( Option::None, false)
            }
        }
    }


    pub fn compute_score(&self, hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
        let winner_response = hand.iter()
            .max_by(|one, other| one.card.cmp(&other.card))
            .unwrap();
        let draw = hand.iter()
            .filter(|response| !(response.card < winner_response.card)).collect::<Vec<_>>();

        let points = TEN_POINTS / draw.len() as i32;
        println!("sending points {}", points);

        for winner_card in draw {
            println!("sending points {}, {}", winner_card.player_id, winner_card.card);
            players[winner_card.player_id as usize].win_points(points);
        }

        match self.round_type {
            RoundType::RUSTIC => {
                players[hand.first().unwrap().player_id as usize].win_points(ONE_POINT);
                players[hand.last().unwrap().player_id as usize].lose_points(FIVE_POINTS);
            }
            _ => {}
        }
        return players;
    }


}
