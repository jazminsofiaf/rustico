use crate::game::round::Round;
use crate::players::player_game::PlayerGame;
use crate::game::rustic_round::RusticRound;

pub struct NormalRound {
    pub len: i32,
    pub forbidden_player_id: Option<i32>,
    pub game_ended: bool,

}

impl NormalRound {
    pub fn new(len: i32, forbidden_player_id: Option<i32>, game_ended: bool ) -> NormalRound {
        NormalRound {
            len,
            forbidden_player_id,
            game_ended,
        }
    }
}

impl Round for NormalRound  {

    fn get_next_rustic_round(&self, number_of_players:i32, _last_player_id: i32)-> RusticRound{
        let round_len = number_of_players;
        RusticRound::new(round_len,  Option::None, false)
    }

    fn get_next_normal_round(&self, number_of_players:i32, _last_player_id: i32)-> NormalRound{
        let round_len = number_of_players;
        NormalRound::new(round_len,  Option::None, false)
    }

    fn wait_turn(&self, player: &PlayerGame){
        player.wait_my_turn();
    }


    fn should_skip_this_round(&self, player: &PlayerGame) -> bool{
        //if(self.base.should_skip_this_round(player)
        match self.forbidden_player_id {
            Some(forbidden_id) if forbidden_id == player.get_id() =>   {
                player.notify_next_player_turn();
                return true;
            }
            _ => {}
        }
        return false;

    }

    fn end_turn(&self, player: &PlayerGame){
        player.notify_next_player_turn();
    }


}
