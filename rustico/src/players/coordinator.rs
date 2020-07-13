use crate::players::player::Player;

use crate::card::french_card::{get_card_deck, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, mpsc, RwLock, Mutex, Condvar};
use std::sync::mpsc::{Receiver, Sender};
use colored::Colorize;
use crate::game::round::{Round, get_random_type_round};
use std::borrow::Borrow;
use rand::thread_rng;
use crate::game::normal_round::NormalRound;


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
        /* to sync start of round.
         * the barrier receives num of players + 1 (for coord.)
         * because the coord. needs to handle each round's setup,
         * and thus also be ready before the round starts.
         * The barrier turns out to be a simple and effective way to sync
         * players and coord.!
         * */
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
        println!("{}", format!("🃏  Coordinator shuffling cards...").bright_white());
        let mut card_deck: Vec<FrenchCard> = get_card_deck();
        card_deck.shuffle(&mut thread_rng());
        return card_deck;
    }

    pub fn deal_cards_between_players(&mut self, cards: Vec<FrenchCard>,
                                      round_info: &Arc<RwLock<Box<dyn Round>>>,
                                      mut turn_to_wait: Arc<(Mutex<bool>, Condvar)>) -> Vec<Player> {
        let amount_of_cards_by_player = cards.len() / self.number_of_players as usize;
        println!("{}", format!("🃏  coordinator dealt {} cards for each player", amount_of_cards_by_player).bright_white());
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
            println!("{}", format!("Cards left after dealing: {:?} ", remaining_cards).bright_white());
        }
        return players;
    }

    pub fn let_the_game_begin(&mut self) {
        println!("{}", "                            🏁  Let the game begin! 🏁".bright_white());
        println!();

        let deck: Vec<FrenchCard> = self.shuffle_deck();
        let number_of_rounds = deck.len() as i32 / self.number_of_players;


        let round_lock: Arc<RwLock<Box<dyn Round>>> = Arc::new(RwLock::new(get_random_type_round()));
        let turn_to_wait = Arc::new((Mutex::new(true), Condvar::new()));
        let turn_coordinator = turn_to_wait.clone();
        let mut players: Vec<Player> = self.deal_cards_between_players(deck, round_lock.borrow(), turn_to_wait);


        for this_round in 0..number_of_rounds {
            /* add 1 to this round so rounds aren't displayed as counting from 0 */
            self.print_round_info(this_round + 1, round_lock.read().unwrap().get_name().to_string());

            self.start_of_round_barrier.wait();

            self.notify_first_turn_start(&*turn_coordinator);

            let mut hand = Vec::new();

            for _ in 0..self.number_of_players {
                let maybe_player_card: Option<PlayerCard> = self.card_receiver.recv().expect("No more cards");
                match maybe_player_card {
                    Some(player_card) => {
                        println!("{}", format!("🃏  [Coordinator] Receiving card: {} from player {}", player_card.card, player_card.player_id).yellow());
                        hand.push(player_card);
                    }
                    _ => {}
                }
            }

            println!("{}", "End of round.".italic().on_red().dimmed().white());
            {
                /* this update occurs here because it is relevant for the next round,
                 * but it must be computed with this round's values
                 * */
                let mut round_info_write_guard = round_lock.write().unwrap();
                (*round_info_write_guard) = round_info_write_guard.get_next_round(hand.last().unwrap().player_id);
                /* ends of block: free lock */
            }

            players = round_lock.read().unwrap().compute_score(hand, players);
        }

        self.end_game(players, round_lock);
    }

    fn notify_first_turn_start(&self, turn_coordinator: &(Mutex<bool>, Condvar)) {
        let (lock, cvar) = turn_coordinator;
        let mut started = lock.lock().unwrap();
        *started = true;
        /* We notify the condvar that the value has changed. */
        println!("{}", format!("🚀  [Coordinator] I declare the game started!").bright_white());
        cvar.notify_one();
    }

    fn end_game(&self, mut players: Vec<Player>, round_lock: Arc<RwLock<Box<dyn Round>>>) {
        {
            /* signal end of game and enable one more round so players can read updated status */
            let mut round_info_write_guard = round_lock.write().unwrap();
            let round = NormalRound::get_end_round();
            *round_info_write_guard = Box::new(round);
        }
        self.start_of_round_barrier.wait();
        self.print_leaderboard(&mut players);

        /* wait for all threads for nice program termination */
        for player in players.iter_mut() {
            player.wait();
        }
    }

    pub fn print_leaderboard(&self, players: &mut Vec<Player>){
        println!();
        println!("{}", format!("                            |-------------+--------|").bold().bright_blue());
        println!("{}", format!("                            |     LEADERBOARD      |").bold().bright_blue());
        println!("{}", format!("                            |-------------+--------|").bold().bright_blue());
        println!("{}", format!("                            |  Player id  | Score  |").bold().bright_blue());
        println!("{}", format!("                            |-------------+--------|").bold().bright_blue());

            for player in players.iter_mut() {
                /* format players id to 2 digits */
                let mut this_players_id = "".to_owned();
                if player.get_id() < 10{
                    this_players_id.push_str(" ");
                }
                this_players_id.push_str(player.get_id().to_string().as_str());

                /* format players score to 3 digits*/
                let mut this_players_score = "".to_owned();
                let score_as_str_len = player.get_points().to_string().len();
                for _ in 0..(3-score_as_str_len) {
                    this_players_score.push_str(" ");
                }
                this_players_score.push_str(player.get_points().to_string().as_str());

                /* print players score */
                println!("{}", format!("                            |     {}      |  {}   |", this_players_id, this_players_score).bold().bright_blue());
        }
        println!("{}", format!("                            |-------------+--------|\n").bold().bright_blue());
    }

    pub fn print_round_info(&self, this_round: i32, game_type: String) {
        let mut _this_round = "".to_owned();
        if this_round < 10 {
            _this_round.push_str("0");
        }

        _this_round.push_str(this_round.to_string().as_str());

        println!("{}", format!("\n|-----------------+--------|\n\
        |     ** NEW ROUND **      |\n\
        |-----------------+--------|\n\
        | num. of round   |   {}   |\n\
        | type            | {} |\n\
        |-----------------+--------|\n", _this_round, game_type).bright_blue());
    }
}