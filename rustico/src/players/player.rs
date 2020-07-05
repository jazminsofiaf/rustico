use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex, Condvar, RwLock};
use crate::card::french_card::FrenchCard;
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;
use crate::players::coordinator::PlayerCard;


pub struct Player {
    thread: Option<thread::JoinHandle<()>>,
    card_receiver: Receiver<PlayerCard>,
    points: i32,
}

impl Player {
    pub fn new(id: i32, cards: Vec<FrenchCard>,
               round_notification: Arc<RwLock<i32>>,
               total_rounds: i32 ) -> Player {
        let (card_sender , card_receiver) = mpsc::channel::<PlayerCard>();
        Player {
            thread: Some(Player::init_play(id, card_sender, cards, round_notification, total_rounds)),
            card_receiver,
            points: 0,
        }
    }

    fn init_play(id: i32,
                 card_sender: Sender<PlayerCard>,
                 mut my_cards: Vec<FrenchCard>,
                 round_notification: Arc<RwLock<i32>>,
                 total_rounds: i32) -> thread::JoinHandle<()>{
        let threadHandler = thread::spawn(move || {

            let mut this_round = 0;
             while this_round < total_rounds {
                 let round = round_notification.read().unwrap();
                 if *round != this_round {
                     continue
                 }
                 println!("[Player {}] round {:?}",id, *round);
                 let first_card: FrenchCard = my_cards.pop().expect("I've no more cards!");
                 println!("[Player {}] sending card {}",id, first_card);
                 let card_to_send = PlayerCard{
                     player_id: id,
                     card: first_card,
                 };
                 card_sender.send(card_to_send).unwrap();
                 this_round = this_round + 1;

            }
        });
        return threadHandler;
    }

    pub fn win_points(&mut self, new_points: i32){
        self.points = self.points + new_points;
    }


    pub fn get_card(&self) -> PlayerCard {
        self.card_receiver.recv().expect("No more cards")
    }

    pub fn wait(&mut self){
        match self.thread.take() {
            Some(th)=> {
                th.join().expect("Error joining the thread");
            },
            _ => {}
        }
    }
}
