use crate::players::round_type::round_type::RoundType;
use rand::{Rng, thread_rng};
use crate::players::player::Player;

use crate::card::french_card::{get_card_dec, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, mpsc, RwLock, Mutex, Condvar};
use std::sync::mpsc::{Receiver, Sender};
use colored::Colorize;
use crate::players::round_type::round_type::RoundType::{RUSTIC, NORMAL};
use crate::game::round::Round;
use std::borrow::{Borrow, BorrowMut};


pub struct PlayerCard {
    pub player_id: i32,
    pub card: FrenchCard,
}

const TEN_POINTS: i32 = 10;
const ONE_POINT: i32 = 1;
const FIVE_POINTS: i32 = 5;

pub struct Coordinator {
    number_of_players: i32,
    start_of_round_barrier: Arc<Barrier>,
    card_sender: Sender<PlayerCard>,
    card_receiver: Receiver<PlayerCard>,
}



impl Coordinator {
    pub fn new(number_of_players: i32) -> Coordinator {
        /* to sync start of round */
        let start_of_round_barrier = Arc::new(Barrier::new(number_of_players as usize + 1));

        let (card_sender , card_receiver) = mpsc::channel::<PlayerCard>();


        return Coordinator {
            number_of_players,
            start_of_round_barrier,
            card_sender,
            card_receiver,
        };
    }

    pub fn get_round_type(&self) -> RoundType {
        let mut rng = rand::thread_rng();
        let round_type: RoundType = rng.gen();
        return round_type;
    }

    pub fn shuffle_deck(&self) -> Vec<FrenchCard> {
        println!("coordinator shuffling cards");
        let mut card_deck: Vec<FrenchCard> = get_card_dec();
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



        ///// definimos info de la primera ronda

        let mut round_type = self.get_round_type();

        let round = Round::new(round_type, Option::None, false);

        let round_info: Arc<RwLock<Round>> = Arc::new(RwLock::new(round));


        let mut turn_to_wait = Arc::new((Mutex::new(true), Condvar::new()));
        let turn_coordinator =turn_to_wait.clone();

        let mut players: Vec<Player> = self.deal_cards_between_players(deck, round_info.borrow(), turn_to_wait);

        let mut round_len =  players.len();

        //////

        for this_round in 0..number_of_rounds {

            println!("{}", format!("** New round! **\n- num of round: {}\n- type = {} ", this_round, round_type).bright_blue());
            println!("from coord. 'bout to start round");
            let barrier_wait_result = self.start_of_round_barrier.wait().is_leader();
            println!("{} from coord.", barrier_wait_result);


            {
                let (lock, cvar) = &*turn_coordinator;
                let mut started = lock.lock().unwrap();
                *started = true;
                // We notify the condvar that the value has changed.
                println!("notificamos que empieza el juego");
                cvar.notify_one();
            }



            let mut hand = Vec::new();

            for _ in 0..round_len {
                let player_card: PlayerCard = self.card_receiver.recv().expect("No more cards");
                println!("receiving card: {} from player {}",player_card.card, player_card.player_id);
                hand.push(player_card);
            }

            println!("{}", "End of round.".bright_red());


            /* this update occurs here because it is relevant for the next round, but it must be
             * computed with this round's values
             */
            let next_round_type = self.get_round_type();
            let mut  round_info_write_guard= round_info.write().unwrap();
            match round_type {
                RUSTIC => {
                    (*round_info_write_guard).forbidden_player_id = Some(hand.last().unwrap().player_id);
                    (*round_info_write_guard).round_type = next_round_type;
                    round_len = players.len() - 1;

                }
                _ => {
                    (*round_info_write_guard).forbidden_player_id = Option::None;
                    (*round_info_write_guard).round_type = next_round_type;
                    round_len = players.len();
                }
            }

            players = self.compute_score(round_type,hand, players);
            round_type = next_round_type;

        }
        self.end_game(players,round_info );



    }

    fn end_game(&self, mut players: Vec<Player>, round_info: Arc<RwLock<Round>>){

        /* signal end of game and enable one more round so players can read updated status */
        {
            let mut round_info_write_guard = round_info.write().unwrap();
            (*round_info_write_guard).game_ended = true;
            self.start_of_round_barrier.wait();
        }

        /* wait for all threads for nice program termination */
        for player in players.iter_mut() {
            println!("player id: {} POINTS = {}",player.get_id(), player.get_points());
            player.wait();
        }

        println!("game ends");

    }

    pub fn compute_score(&self,round_type: RoundType ,hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
        let winner_response = hand.iter()
            .max_by(|one, other| one.card.cmp(&other.card))
            .unwrap();
        let draw = hand.iter()
            .filter(|response| !(response.card < winner_response.card)).collect::<Vec<_>>();

        let points = TEN_POINTS / draw.len() as i32;
        println!("sending points {}", points);

        for winner_card in draw {
            println!("sending points {}, {}", winner_card.player_id, winner_card.card);
            players[winner_card.player_id as usize].win_points(points);
        }

        match round_type {
            RUSTIC => {
                players[hand.first().unwrap().player_id as usize].win_points(ONE_POINT);
                players[hand.last().unwrap().player_id as usize].lose_points(FIVE_POINTS);
            }
            _ => {}
        }
        return players;


    }
}