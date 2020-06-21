use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use crate::card::{FrenchCard, CardSuit, CardNumber};

pub struct Player {
    thread: Option<thread::JoinHandle<()>>,
    card_receiver: Receiver<FrenchCard>,
}

impl Player {
    pub fn new() -> Player {
        let (card_sender , card_receiver) = mpsc::channel::<FrenchCard>();


        let thread = thread::spawn(move || {
            //TODO use condvar while(continue playing)
            for _x in 0..2 {
                println!("playing ");

                //TODO get the max card
                let card :FrenchCard = FrenchCard::new(CardSuit::CLOVER,  CardNumber::TWO );
                card_sender.send(card).unwrap();
            }
        });

        Player {
            thread: Some(thread),
            card_receiver
        }
    }

    pub fn getCard(&self) -> FrenchCard {
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
