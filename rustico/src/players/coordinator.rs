use crate::players::round_type::round_type::RoundType;
use rand::{Rng, thread_rng};
use crate::players::player::Player;
use crate::card::french_card::{get_card_dec, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, Mutex, Condvar, RwLock};
use std::borrow::Borrow;
use colored::Colorize;

pub struct PlayerCard {
    pub player_id: i32,
    pub card: FrenchCard,
}

const TEN_POINTS: i32 = 10;

pub struct Coordinator {
    number_of_players: i32,
    barrier: Arc<Barrier>,
    round_notification: Arc<(Mutex<(bool, i32)>, Condvar)>,
}

impl Coordinator {
    pub fn new(number_of_players: i32) -> Coordinator {
        /* to know who was the last player to lay a card down */
        let barrier = Arc::new(Barrier::new(number_of_players as usize));

        /* Condvar is a tuple (bool, int), where
         *          - bool indicates whether the round can start or not, and
         *          - the int indicates the round to play.
         * Previous approach made use only of a bool, but it lacked precision.
         * */
        let round_notification = Arc::new((Mutex::new((false, -1)), Condvar::new()));

        return Coordinator {
            number_of_players,
            barrier,
            round_notification,
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


    pub fn deal_cards_between_players(&self, cards: Vec<FrenchCard>) -> Vec<Player> {
        let number_of_rounds = cards.len() as i32 / self.number_of_players;
        let amount_of_cards_by_player = cards.len() / self.number_of_players as usize;
        println!("coordinator dealt {} cards for each player", amount_of_cards_by_player);
        let mut card_iter = cards.into_iter().peekable();


        let mut players: Vec<Player> = Vec::with_capacity(self.number_of_players as usize);
        for player_id in 0..self.number_of_players {
            let cards_for_player: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
            let player: Player = Player::new(player_id,
                                             cards_for_player,
                                             self.round_notification.clone(),
                                             number_of_rounds,
                                             self.barrier.clone());
            players.push(player);
        }

        let remaining_cards: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
        if remaining_cards.len() > 0 {
            println!("Cards left after dealing: {:?} ", remaining_cards);
        }
        return players;
    }


    pub fn let_the_game_begin(&self) {
        println!("{}", "Let the game begin!".bright_white());

        let deck: Vec<FrenchCard> = self.shuffle_deck();
        let number_of_rounds = deck.len() as i32 / self.number_of_players;
        let mut players: Vec<Player> = self.deal_cards_between_players(deck);

        let (mtx, cnd) = &*self.round_notification;
        // let &(ref mtx, ref cnd) = &*self.round_notification;

        for this_round in 0..number_of_rounds {
            let round_type = self.get_round_type();

            // println!("** New round! **\n- num of round: {}\n- type = {} ", this_round, round_type);
            println!("{}", format!("** New round! **\n- num of round: {}\n- type = {} ", this_round, round_type).bright_blue());

            {
                let mut guard = mtx.lock().unwrap();
                // guard.1 = guard.1 + 1;
                guard.1 = guard.1.wrapping_add(1);
                guard.0 = true;
                cnd.notify_all();
                println!("{}", "New round started!".red());
            }

            let mut hand = Vec::new();
            for player in players.iter() {
                let player_card: PlayerCard = player.get_card();
                println!("receiving card: {} from player {}", player_card.card, player_card.player_id);
                hand.push(player_card);
            }

            {
                let mut guard = mtx.lock().unwrap();
                guard.0 = false;
                cnd.notify_all();
            }

            println!("{}", "End of round.".bright_red());
        }

        for player in players.iter_mut() {
            player.wait();
        }

        println!("game ends");
    }

    pub fn compute_score(&self, hand: Vec<PlayerCard>, mut players: Vec<Player>) -> Vec<Player> {
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
        return players;
    }
}