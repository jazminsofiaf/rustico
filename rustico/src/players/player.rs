use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex, Condvar, RwLock};
use crate::card::french_card::FrenchCard;
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;


pub struct Player {
    thread: Option<thread::JoinHandle<()>>,
    card_receiver: Receiver<(i32, FrenchCard)>,
}

impl Player {
    pub fn new(id: i32, cards: Vec<FrenchCard>,
               round_notification: Arc<RwLock<i32>>,
               total_rounds: i32 ) -> Player {
        let (card_sender , card_receiver) = mpsc::channel::<(i32, FrenchCard)>();
        Player {
            thread: Some(Player::init_play(id, card_sender, cards, round_notification, total_rounds)),
            card_receiver,
        }
    }

    fn init_play(id: i32,
                 card_sender: Sender<(i32, FrenchCard)>,
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
                 card_sender.send((id, first_card) ).unwrap();
                 this_round = this_round + 1;

            }
        });
        return threadHandler;
    }


    pub fn get_card(&self) -> (i32, FrenchCard) {
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
