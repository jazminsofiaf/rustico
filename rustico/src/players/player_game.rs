use std::sync::mpsc::Sender;
use std::sync::{Barrier, Arc, Mutex, Condvar, RwLock};
use crate::card::french_card::FrenchCard;
use crate::players::coordinator::PlayerCard;
use crate::game::round::Round;
use crate::players::round_type::round_type::RoundType;
use colored::Colorize;
use std::borrow::{Borrow, BorrowMut};

pub struct PlayerGame{
    id: i32,
    card_sender: Sender<PlayerCard>,
    my_cards: Vec<FrenchCard>,
    start_of_round_barrier: Arc<Barrier>,
    my_turn: Arc<(Mutex<bool>, Condvar)>,
    next_turn: Arc<(Mutex<bool>, Condvar)>,
    round_info: Arc<RwLock<Round>>
}

impl PlayerGame {

    pub fn new(id: i32,
               card_sender: Sender<PlayerCard>,
               my_cards: Vec<FrenchCard>,
               start_of_round_barrier: Arc<Barrier>,
               my_turn: Arc<(Mutex<bool>, Condvar)>,
               next_turn: Arc<(Mutex<bool>, Condvar)>,
               round_info: Arc<RwLock<Round>>) -> PlayerGame {
        PlayerGame {
            id,
            card_sender,
            my_cards,
            start_of_round_barrier,
            my_turn,
            next_turn,
            round_info,
        }
    }

    pub fn init(&mut self) {
        loop {
            println!("{}", format!("[Player {}] waiting to start round", self.id).dimmed().red());
            let barrier_wait_result = self.start_of_round_barrier.wait().is_leader();
            println!("[Player {}]  barrier_wait_result {} ", self.id, barrier_wait_result);

            let mut end_game = false;
            let mut forbidden_player_id = Option::None;
            let mut round_type = RoundType::NORMAL;
            self.get_round_info(end_game.borrow_mut(), forbidden_player_id.borrow_mut(), round_type.borrow_mut());
            if end_game {
                break;
            }

            match round_type {
                RoundType::NORMAL => {
                    self.wait_my_turn();
                    if self.should_skip_this_turn(forbidden_player_id) {
                        self.notify_next_player_turn();
                        continue;
                    }
                    self.play_this_round();
                    self.notify_next_player_turn();
                }
                RoundType::RUSTIC=> {
                    if self.should_skip_this_turn(forbidden_player_id) {
                        continue;
                    }
                    self.play_this_round();
                }

            }
        }
    }

    fn get_round_info(&self,  end_game: &mut bool,  forbidden_player_id:  &mut Option<i32>, round_type: &mut RoundType  ){
        let round_info_res = self.round_info.read().unwrap();
        if (*round_info_res).game_ended {
            println!("end of game reached in player");
            *end_game = true;
            return;
        }
        *forbidden_player_id = (*round_info_res).forbidden_player_id;
        *round_type = (*round_info_res).round_type;

    }

    fn wait_my_turn(&self){
        let (lock, cvar) = &*self.my_turn;
        let mut is_my_turn = lock.lock().unwrap();
        println!("[player {}]ya es mi turno? {}",  self.id, is_my_turn);
        while !*is_my_turn {
            is_my_turn = cvar.wait(is_my_turn).unwrap();
        }
        //reinicio para las proximas rondas
        *is_my_turn = false;
    }

    fn notify_next_player_turn(&self){
        let (lock, cvar) = &*self.next_turn;
        let mut my_turn_end = lock.lock().unwrap();
        *my_turn_end = true;
        // We notify the condvar that the next turn has started.
        cvar.notify_all();
    }

    fn play_this_round(&mut self){
        let first_card: FrenchCard = self.my_cards.pop().expect("I've no more cards!");
        println!("{}", format!("[Player {}] sending card {}", self.id, first_card).bright_magenta());
        let card_to_send = PlayerCard {
            player_id: self.id,
            card: first_card,
        };
        self.card_sender.send(card_to_send).unwrap();
    }

    fn should_skip_this_turn(&self,forbidden_player: Option<i32> ) -> bool {
        /* skip round if I put the last card in prev. round, which was rustic */
        match forbidden_player {
            Some(forbidden_player_id) if forbidden_player_id == self.id =>   {
                println!("{}", format!("[Player {}] skip round cuz I put the last card in prev. round, which was rustic", self.id).dimmed().bright_magenta());
                return true;
            }
            _ => {}
        }
        return false;
    }
}

