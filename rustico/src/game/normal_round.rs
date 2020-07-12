use crate::game::round::Round;
use crate::players::player_game::PlayerGame;
use crate::game::rustic_round::RusticRound;

pub struct NormalRound {
    forbidden_player_id: Option<i32>,
    game_ended: bool,

}

impl NormalRound {
    pub fn new(forbidden_player_id: Option<i32>, game_ended: bool ) -> NormalRound {
        NormalRound {
            forbidden_player_id,
            game_ended,
        }
    }
}

impl Round for NormalRound  {
    fn get_forbidden_player_id(&self) -> Option<i32> {
        return self.forbidden_player_id;
    }

    fn is_game_ended(&self) -> bool {
       return self.game_ended;
    }


    fn get_next_rustic_round(&self, _last_player_id: i32)-> RusticRound{
        RusticRound::new(  Option::None, false)
    }

    fn get_next_normal_round(&self,  _last_player_id: i32)-> NormalRound{
        NormalRound::new( Option::None, false)
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
