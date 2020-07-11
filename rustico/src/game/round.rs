use crate::players::round_type::round_type::RoundType;
use crate::players::coordinator::PlayerCard;
use crate::players::player::Player;

const TEN_POINTS: i32 = 10;
const ONE_POINT: i32 = 1;
const FIVE_POINTS: i32 = 5;

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


    pub fn compute_score(&self,hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
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
            RUSTIC => {
                players[hand.first().unwrap().player_id as usize].win_points(ONE_POINT);
                players[hand.last().unwrap().player_id as usize].lose_points(FIVE_POINTS);
            }
            _ => {}
        }
        return players;

    }
}
