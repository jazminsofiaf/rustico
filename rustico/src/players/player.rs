use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Barrier, RwLock, Mutex, Condvar};
use crate::card::french_card::FrenchCard;
use crate::players::coordinator::PlayerCard;
use colored::Colorize;
use crate::game::round::Round;
use std::borrow::Borrow;
use crate::players::round_type::round_type::RoundType;


pub struct Player {
    id: i32,
    thread: Option<thread::JoinHandle<()>>,
    points: i32,
}

impl Player {
    pub fn new(id: i32,
               card_sender: Sender<PlayerCard>,
               cards: Vec<FrenchCard>,
               start_of_round_barrier: Arc<Barrier>,
               my_turn: Arc<(Mutex<bool>, Condvar)>,
               next_turn: Arc<(Mutex<bool>, Condvar)>,
               round_info: Arc<RwLock<Round>>) -> Player {
        Player {
            id,
            thread: Some(Player::init_play(id, card_sender, cards, start_of_round_barrier,my_turn, next_turn, round_info)),
            points: 0,

        }
    }

    pub fn get_id(&self) ->i32{
        return self.id;
    }


    fn init_play(id: i32,
                 card_sender: Sender<PlayerCard>,
                 mut my_cards: Vec<FrenchCard>,
                 barrier: Arc<Barrier>,
                 my_turn: Arc<(Mutex<bool>, Condvar)>,
                 next_turn: Arc<(Mutex<bool>, Condvar)>,
                 round_info: Arc<RwLock<Round>>) -> thread::JoinHandle<()> {
        let thread_handler = thread::spawn(move || {
            loop {
                println!("{}", format!("[Player {}] waiting to start round", id).dimmed().red());
                let barrier_wait_result = barrier.wait().is_leader();
                println!("[Player {}]  barrier_wait_result {} ", id, barrier_wait_result);

                let round_info_res = round_info.read().unwrap();
                if (*round_info_res).game_ended {
                    println!("end of game reached in player");
                    break;
                }


                match (*round_info_res).round_type {
                    RoundType::NORMAL =>   {

                        // Wait for this turn to start------------------------------
                        let (lock, cvar) = &*my_turn;
                        let mut is_my_turn = lock.lock().unwrap();
                        println!("[player {}]ya es mi turno? {}", id, is_my_turn);
                        while !*is_my_turn {
                            is_my_turn = cvar.wait(is_my_turn).unwrap();
                        }
                        //reinicio para las proximas rondas
                        *is_my_turn = false;
                        // Wait for this turn to start------------------------------


                        //-skip------------------------------------------------------------------------------------------
                        /* skip round if I put the last card in prev. round, which was rustic */
                        match (*round_info_res).forbidden_player_id {
                            Some(forbidden_player_id) if forbidden_player_id == id =>   {
                                println!("{}", format!("[Player {}] skip round cuz I put the last card in prev. round, which was rustic", id).dimmed().bright_magenta());

                                // My turn end ---------------------------------------
                                let (lock, cvar) = &*next_turn;
                                let mut my_turn_end = lock.lock().unwrap();
                                *my_turn_end = true;
                                // We notify the condvar that the next turn has started.
                                cvar.notify_all();
                                // My turn end ---------------------------------------


                                continue;

                            }
                            _ => {}
                        }
                        //-skip------------------------------------------------------------------------------------------


                        //play -------------------------------------------------------------------------------------
                        let first_card: FrenchCard = my_cards.pop().expect("I've no more cards!");
                        println!("{}", format!("[Player {}] sending card {}", id, first_card).bright_magenta());
                        let card_to_send = PlayerCard {
                            player_id: id,
                            card: first_card,
                        };
                        card_sender.send(card_to_send).unwrap();
                        //play ---------------------------------------------------------------------------------------


                        // My turn end ---------------------------------------
                        let (lock, cvar) = &*next_turn;
                        let mut my_turn_end = lock.lock().unwrap();
                        *my_turn_end = true;
                        // We notify the condvar that the next turn has started.
                        cvar.notify_all();
                        // My turn end ---------------------------------------

                    }

                    //RUSTICOOOOOO
                    _ => {

                        //-skip -----------------------------------------------------------------------------------------
                        /* skip round if I put the last card in prev. round, which was rustic */
                        match (*round_info_res).forbidden_player_id {
                            Some(forbidden_player_id) if forbidden_player_id == id =>   {
                                println!("{}", format!("[Player {}] skip round cuz I put the last card in prev. round, which was rustic", id).dimmed().bright_magenta());
                                continue;
                            }
                            _ => {}
                        }
                        //-skip ------------------------------------------------------------------------------------------


                        //play -------------------------------------------------------------------------------------
                        let first_card: FrenchCard = my_cards.pop().expect("I've no more cards!");
                        println!("{}", format!("[Player {}] sending card {}", id, first_card).bright_magenta());
                        let card_to_send = PlayerCard {
                            player_id: id,
                            card: first_card,
                        };
                        card_sender.send(card_to_send).unwrap();
                        //play ---------------------------------------------------------------------------------------
                    }
                }

            }
        });
        return thread_handler;
    }


    pub fn win_points(&mut self, new_points: i32) {
        self.points = self.points + new_points;
    }
    pub fn lose_points(&mut self, new_points: i32) {
        self.points = self.points - new_points;
    }

    pub fn get_points(&self) -> i32 {
        return self.points;
    }

    pub fn wait(&mut self) {
        match self.thread.take() {
            Some(th) => {
                th.join().expect("Error joining the thread");
            }
            _ => {}
        }
    }
}
