use std::thread;
use std::sync::mpsc::{Sender};
use std::sync::{Arc, Mutex, Condvar};
use crate::card::french_card::FrenchCard;
use crate::players::coordinator::PlayerCard;
use colored::Colorize;


pub struct Player {
    thread: Option<thread::JoinHandle<()>>,
    points: i32,
}

impl Player {
    pub fn new(id: i32,
               card_sender: Sender<PlayerCard>,
               cards: Vec<FrenchCard>,
               round_notification: Arc<(Mutex<(bool, i32)>, Condvar)>,
               total_rounds: i32) -> Player {
        Player {
            thread: Some(Player::init_play(id, card_sender, cards, round_notification, total_rounds)),
            points: 0,
        }
    }


    fn init_play(id: i32,
                 card_sender: Sender<PlayerCard>,
                 mut my_cards: Vec<FrenchCard>,
                 round_notification: Arc<(Mutex<(bool, i32)>, Condvar)>,
                 total_rounds: i32) -> thread::JoinHandle<()> {
        let thread_handler = thread::spawn(move || {
            let &(ref mtx, ref cnd) = &*round_notification;

            let mut this_round = 0;
            while this_round < total_rounds {
                let mut round_has_started = mtx.lock().unwrap();
                while !round_has_started.0 {
                    println!("{}", format!("[Player {}] waiting to start round {}", id, this_round).dimmed().red());
                    round_has_started = cnd.wait(round_has_started).unwrap();
                }
                if round_has_started.1 != this_round {
                    continue;
                }

                println!("{}", format!("[Player {}] round {}", id, this_round).italic().cyan());
                let first_card: FrenchCard = my_cards.pop().expect("I've no more cards!");
                println!("{}", format!("[Player {}] sending card {}", id, first_card).bright_magenta());
                let card_to_send = PlayerCard {
                    player_id: id,
                    card: first_card,
                };
                card_sender.send(card_to_send).unwrap();
                // let barrier_wait_result = barrier.wait().is_leader();
                // println!("{}", barrier_wait_result);
                this_round = this_round + 1;
            }
        });
        return thread_handler;
    }

    pub fn win_points(&mut self, new_points: i32) {
        self.points = self.points + new_points;
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
