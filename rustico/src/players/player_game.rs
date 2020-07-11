use std::sync::mpsc::Sender;
use std::sync::{Barrier, Arc, Mutex, Condvar, RwLock};
use crate::card::french_card::FrenchCard;
use crate::players::coordinator::PlayerCard;
use crate::game::round::Round;
use colored::Colorize;
use std::borrow::Borrow;

pub struct PlayerGame{
    id: i32,
    card_sender: Sender<PlayerCard>,
    my_cards: Vec<FrenchCard>,
    start_of_round_barrier: Arc<Barrier>,
    my_turn: Arc<(Mutex<bool>, Condvar)>,
    next_turn: Arc<(Mutex<bool>, Condvar)>,
    round_lock: Arc<RwLock<dyn Round>>
}

impl PlayerGame {

    pub fn new(id: i32,
               card_sender: Sender<PlayerCard>,
               my_cards: Vec<FrenchCard>,
               start_of_round_barrier: Arc<Barrier>,
               my_turn: Arc<(Mutex<bool>, Condvar)>,
               next_turn: Arc<(Mutex<bool>, Condvar)>,
               round_info: Arc<RwLock<dyn Round>>) -> PlayerGame {
        PlayerGame {
            id,
            card_sender,
            my_cards,
            start_of_round_barrier,
            my_turn,
            next_turn,
            round_lock: round_info,
        }
    }

    pub fn init(&mut self) {
        loop {
            println!("{}", format!("[Player {}] waiting to start round", self.id).dimmed().red());
            let barrier_wait_result = self.start_of_round_barrier.wait().is_leader();
            println!("[Player {}] barrier result {} ", self.id, barrier_wait_result);

            if self.round_lock.read().unwrap().game_ended {
                break;
            }

            self.round_lock.read().unwrap().wait_turn(self.borrow());
            if self.round_lock.read().unwrap().should_skip_this_round(self.borrow()){
                println!("{}", format!("[Player {}] skip round cuz I put the last card in prev. round, which was rustic", self.id).dimmed().bright_magenta());
                continue;
            }
            self.play_this_round();
            self.round_lock.read().unwrap().end_turn(self.borrow());
        }
    }

    pub fn get_id(&self) ->i32{
        return self.id;
    }


    pub(crate) fn wait_my_turn(&self){
        let (lock, cvar) = &*self.my_turn;
        let mut is_my_turn = lock.lock().unwrap();
        println!("[player {}] is it my turn already ? {}",  self.id, is_my_turn);
        while !*is_my_turn {
            is_my_turn = cvar.wait(is_my_turn).unwrap();
        }
        //reinicio para las proximas rondas
        *is_my_turn = false;
    }

    pub fn notify_next_player_turn(&self){
        let (lock, cvar) = &*self.next_turn;
        let mut my_turn_end = lock.lock().unwrap();
        *my_turn_end = true;
        // We notify the condvar that the next turn has started.
        cvar.notify_all();
    }

    pub fn play_this_round(&mut self){
        let first_card: FrenchCard = self.my_cards.pop().expect("I've no more cards!");
        println!("{}", format!("[Player {}] sending card {}", self.id, first_card).bright_magenta());
        let card_to_send = PlayerCard {
            player_id: self.id,
            card: first_card,
        };
        self.card_sender.send(card_to_send).unwrap();
    }

}

