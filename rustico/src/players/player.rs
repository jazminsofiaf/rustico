use std::thread;
use std::sync::mpsc::{Sender};
use std::sync::{Arc, Barrier, RwLock};
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
               start_of_round_barrier: Arc<Barrier>,
               round_info: Arc<RwLock<u32>>) -> Player {
        Player {
            thread: Some(Player::init_play(id, card_sender, cards, start_of_round_barrier, round_info)),
            points: 0,
        }
    }


    fn init_play(id: i32,
                 card_sender: Sender<PlayerCard>,
                 mut my_cards: Vec<FrenchCard>,
                 barrier: Arc<Barrier>,
                 round_info: Arc<RwLock<u32>>) -> thread::JoinHandle<()> {
        let thread_handler = thread::spawn(move || {
            loop {
                println!("{}", format!("[Player {}] waiting to start round", id).dimmed().red());
                let barrier_wait_result = barrier.wait().is_leader();

                let round_info_res = round_info.read().unwrap();
                println!("{}", *round_info_res);
                if *round_info_res == 99 {
                    println!("end of game reached in player");
                    break;
                }

                let first_card: FrenchCard = my_cards.pop().expect("I've no more cards!");
                println!("{}", format!("[Player {}] sending card {}", id, first_card).bright_magenta());
                let card_to_send = PlayerCard {
                    player_id: id,
                    card: first_card,
                };
                card_sender.send(card_to_send).unwrap();
                println!("{} from player {}", barrier_wait_result, id);
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
