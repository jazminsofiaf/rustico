use crate::players::round_type::round_type::RoundType;
use rand::{Rng, thread_rng};
use crate::players::player::Player;
use crate::card::french_card::{get_card_dec, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, mpsc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use colored::Colorize;

pub struct PlayerCard {
    pub player_id: i32,
    pub card: FrenchCard,
}

const TEN_POINTS: i32 = 10;

pub struct Coordinator {
    number_of_players: i32,
    start_of_round_barrier: Arc<Barrier>,
    card_sender: Sender<PlayerCard>,
    card_receiver: Receiver<PlayerCard>,
    round_info: Arc<RwLock<u32>>,
}

impl Coordinator {
    pub fn new(number_of_players: i32) -> Coordinator {
        /* to sync start of round */
        let start_of_round_barrier = Arc::new(Barrier::new(number_of_players as usize + 1));

        /* To specify which players shouldn't play next round.
         * Below, table with values (protocol):
         *          - player id: specified player shouldn't play this round
         *          - 50: all players should play
         *          - 99: game ended
         * */
        let round_info: Arc<RwLock<u32>> = Arc::new(RwLock::new(50));

        let (card_sender, card_receiver) = mpsc::channel::<PlayerCard>();

        return Coordinator {
            number_of_players,
            start_of_round_barrier,
            card_sender,
            card_receiver,
            round_info,
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
        let amount_of_cards_by_player = cards.len() / self.number_of_players as usize;
        println!("coordinator deal {} cards for each player", amount_of_cards_by_player);
        let mut card_iter = cards.into_iter().peekable();


        let mut players: Vec<Player> = Vec::with_capacity(self.number_of_players as usize);
        for player_id in 0..self.number_of_players {
            let cards_for_player: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
            let player: Player = Player::new(player_id,
                                             self.card_sender.clone(),
                                             cards_for_player,
                                             self.start_of_round_barrier.clone(),
                                             Arc::clone(&self.round_info));
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

        for this_round in 0..number_of_rounds {
            let round_type = self.get_round_type();

            println!("{}", format!("** New round! **\n- num of round: {}\n- type = {} ", this_round, round_type).bright_blue());

            {
                let mut w = self.round_info.write().unwrap();
                // TODO actualizar con id correspondiente en caso que un jugador haya llegado ultimo
                *w = 50;
            }

            println!("from coord. 'bout to start round");
            let barrier_wait_result = self.start_of_round_barrier.wait().is_leader();
            println!("{} from coord.", barrier_wait_result);

            let mut hand = Vec::new();
            for _player in players.iter() {
                let player_card: PlayerCard = self.card_receiver.recv().expect("No more cards");
                println!("receiving card: {} from player {}", player_card.card, player_card.player_id);
                hand.push(player_card);
            }

            // TODO computo puntaje.

            println!("{}", "End of round.".bright_red());
        }


        /* signal end of game and enable one more round so players can read updated status */
        {
            let mut w = self.round_info.write().unwrap();
            *w = 99;
            self.start_of_round_barrier.wait();
        }

        /* wait for all threads for nice program termination */
        for player in players.iter_mut() {
            player.wait();
        }

        println!("game ends");
        //     TODO Print leaderboard.
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