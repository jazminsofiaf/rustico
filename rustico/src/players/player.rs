use std::thread;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Barrier, RwLock, Mutex, Condvar};
use crate::card::french_card::FrenchCard;
use crate::players::coordinator::PlayerCard;
use crate::game::round::Round;
use crate::players::player_game::PlayerGame;


pub struct Player {
    id: i32,
    thread: Option<thread::JoinHandle<()>>,
    points: i32,
}

impl Player {
    pub fn new(id: i32,
               card_sender: Sender<Option<PlayerCard>>,
               cards: Vec<FrenchCard>,
               start_of_round_barrier: Arc<Barrier>,
               my_turn: Arc<(Mutex<bool>, Condvar)>,
               next_turn: Arc<(Mutex<bool>, Condvar)>,
               round_info: Arc<RwLock<Box<dyn Round>>>) -> Player {
        Player {
            id,
            thread: Some(Player::init_play(id, card_sender, cards, start_of_round_barrier, my_turn, next_turn, round_info)),
            points: 0,

        }
    }

    pub fn get_id(&self) -> i32 {
        return self.id;
    }


    fn init_play(id: i32,
                 card_sender: Sender<Option<PlayerCard>>,
                 my_cards: Vec<FrenchCard>,
                 barrier: Arc<Barrier>,
                 my_turn: Arc<(Mutex<bool>, Condvar)>,
                 next_turn: Arc<(Mutex<bool>, Condvar)>,
                 round_info: Arc<RwLock<Box<dyn Round>>>) -> thread::JoinHandle<()> {

        let thread_handler = thread::spawn(move || {
            let mut player_game = PlayerGame::new(id, card_sender, my_cards, barrier, my_turn, next_turn, round_info);
            player_game.init();
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
