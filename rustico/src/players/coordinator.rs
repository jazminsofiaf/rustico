use crate::players::player::Player;

use crate::card::french_card::{get_card_deck, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, mpsc, RwLock, Mutex, Condvar};
use std::sync::mpsc::{Receiver, Sender};
use colored::Colorize;
use crate::game::round::Round;
use std::borrow::Borrow;
use rand::thread_rng;


pub struct PlayerCard {
    pub player_id: i32,
    pub card: FrenchCard,
}


pub struct Coordinator {
    number_of_players: i32,
    start_of_round_barrier: Arc<Barrier>,
    card_sender: Sender<Option<PlayerCard>>,
    card_receiver: Receiver<Option<PlayerCard>>,
}


impl Coordinator {
    pub fn new(number_of_players: i32) -> Coordinator {
        /* to sync start of round */
        let start_of_round_barrier = Arc::new(Barrier::new(number_of_players as usize + 1));

        let (card_sender, card_receiver) = mpsc::channel::<Option<PlayerCard>>();

        return Coordinator {
            number_of_players,
            start_of_round_barrier,
            card_sender,
            card_receiver,
        };
    }


    pub fn shuffle_deck(&self) -> Vec<FrenchCard> {
        println!("coordinator shuffling cards");
        let mut card_deck: Vec<FrenchCard> = get_card_deck();
        card_deck.shuffle(&mut thread_rng());
        return card_deck;
    }


    pub fn deal_cards_between_players(&mut self, cards: Vec<FrenchCard>, round_info: &Arc<RwLock<Round>>, mut turn_to_wait: Arc<(Mutex<bool>, Condvar)>) -> Vec<Player> {
        let amount_of_cards_by_player = cards.len() / self.number_of_players as usize;
        println!("coordinator deal {} cards for each player", amount_of_cards_by_player);
        let mut card_iter = cards.into_iter().peekable();


        let mut players: Vec<Player> = Vec::with_capacity(self.number_of_players as usize);
        for player_id in 0..self.number_of_players {
            let turn = Arc::new((Mutex::new(false), Condvar::new()));
            let next_turn = turn.clone();

            let cards_for_player: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
            let player: Player = Player::new(player_id,
                                             self.card_sender.clone(),
                                             cards_for_player,
                                             self.start_of_round_barrier.clone(),
                                             turn_to_wait,
                                             next_turn,
                                             Arc::clone(round_info));
            players.push(player);
            turn_to_wait = turn;
        }

        let remaining_cards: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
        if remaining_cards.len() > 0 {
            println!("Cards left after dealing: {:?} ", remaining_cards);
        }
        return players;
    }


    pub fn let_the_game_begin(&mut self) {
        println!("{}", "Let the game begin!".bright_white());

        let deck: Vec<FrenchCard> = self.shuffle_deck();
        let number_of_rounds = deck.len() as i32 / self.number_of_players;


        let round = Round::new(Option::None, false);
        let round_lock: Arc<RwLock<Round>> = Arc::new(RwLock::new(round));

        let turn_to_wait = Arc::new((Mutex::new(true), Condvar::new()));
        let turn_coordinator = turn_to_wait.clone();
        let mut players: Vec<Player> = self.deal_cards_between_players(deck, round_lock.borrow(), turn_to_wait);


        for this_round in 0..number_of_rounds {
            println!("{}", format!("** New round! **\n- num of round: {}\n- type = {} ", this_round, round_lock.read().unwrap().round_type).bright_blue());
            let barrier_wait_result = self.start_of_round_barrier.wait().is_leader();
            println!("[Coordinator] barrier result: {}", barrier_wait_result);

            self.notify_first_turn_start(&*turn_coordinator);

            let mut hand = Vec::new();

            for _ in 0..self.number_of_players {
                let maybe_player_card: Option<PlayerCard> = self.card_receiver.recv().expect("No more cards");
                match maybe_player_card {
                    Some(player_card) => {
                        println!("receiving card: {} from player {}", player_card.card, player_card.player_id);
                        hand.push(player_card);
                    }
                    _ => {}
                }
            }

            println!("{}", "End of round.".bright_red());
            {
                //this update occurs here because it is relevant for the next round, but it must be computed with this round's values
                let mut round_info_write_guard = round_lock.write().unwrap();
                (*round_info_write_guard) = round_info_write_guard.get_next_round(hand.last().unwrap().player_id);
                //ends of block free lock
            }

            players = round_lock.read().unwrap().compute_score(hand, players);
        }
        {
            println!("write round lock");
            /* signal end of game and enable one more round so players can read updated status */
            let mut round_info_write_guard = round_lock.write().unwrap();
            println!("sarasa1");
            let round = Round::new(Option::None, true);
            *round_info_write_guard = round;
            // (*round_info_write_guard).game_ended = true;
            println!("has ended? from coord {}", (*round_info_write_guard).game_ended);
            println!("sarasa2");
            // self.notify_first_turn_start(&*turn_coordinator);
            println!("sarasa3");
            //ends of block free lock
            println!("free round round lock");
        }
        self.start_of_round_barrier.wait();

        self.end_game(players, round_lock);
    }


    fn notify_first_turn_start(&self, turn_coordinator: &(Mutex<bool>, Condvar)) {
        let (lock, cvar) = turn_coordinator;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        println!("notificamos que empieza el juego");
        cvar.notify_one();
    }

    fn end_game(&self, mut players: Vec<Player>, _round_lock: Arc<RwLock<Round>>) {
        /* wait for all threads for nice program termination */
        for player in players.iter_mut() {
            println!("player id: {} POINTS = {}", player.get_id(), player.get_points());
            player.wait();
        }
        println!("game ends");
    }
}