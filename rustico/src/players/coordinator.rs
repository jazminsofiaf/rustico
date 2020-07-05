use crate::players::round_type::round_type::RoundType;
use rand::{Rng, thread_rng};
use crate::players::player::Player;
use crate::card::french_card::{get_card_dec, FrenchCard};
use rand::seq::SliceRandom;
use std::sync::{Arc, Barrier, Mutex, Condvar, RwLock};


pub struct Coordinator {
     number_of_players: i32,
     barrier: Arc<Barrier>,
     round_notification: Arc<RwLock<i32>>
}

impl Coordinator {
    pub fn new(number_of_players: i32) ->  Coordinator {

        /* to know who was the last player to lay a card down */
        let barrier = Arc::new(Barrier::new(number_of_players as usize));

        let round_notification = Arc::new(RwLock::new(-1));

        return Coordinator {
            number_of_players,
            barrier,
            round_notification,
        };
    }

    pub fn get_round_type(&self) -> RoundType {
        let mut rng = rand::thread_rng();
        let round_type: RoundType = rng.gen();
        return  round_type;
    }

    pub fn shuffle_deck(&self)-> Vec<FrenchCard> {
        println!("coordinator shaffle cards");
        let mut card_deck: Vec<FrenchCard> = get_card_dec();
        card_deck.shuffle(&mut thread_rng());
        return card_deck;
    }


    pub fn deal_cards_between_players(&self, cards : Vec<FrenchCard>) ->  Vec<Player>{

        let number_of_rounds = cards.len() as i32/ self.number_of_players;
        let amount_of_cards_by_player = cards.len() / self.number_of_players as usize;
        println!("coordinator deal {} cards for each player", amount_of_cards_by_player);
        let mut card_iter = cards.into_iter().peekable();

        let mut players: Vec<Player>= Vec::with_capacity(self.number_of_players as usize);
            for player_id in 0..self.number_of_players {
                let cards_for_player: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
                let player: Player = Player::new(player_id, cards_for_player, self.round_notification.clone(), number_of_rounds);
                players.push(player);
            }
        let remaining_cards: Vec<FrenchCard> =  card_iter.by_ref().take(amount_of_cards_by_player).collect();
        println!("remaining cards  {:?} ", remaining_cards);
        return players;
    }



    pub fn let_the_game_begin(&self){
        println!("let the games begin");

        let deck : Vec<FrenchCard> = self.shuffle_deck();
        let number_of_rounds = deck.len() as i32 / self.number_of_players;
        let players: Vec<Player> = self.deal_cards_between_players(deck);


        for this_round in 0..number_of_rounds {
            let round_type= self.get_round_type();
            println!("new round : {}, type = {} ", this_round, round_type);

            {   // RAII lock free in end block
                let mut round_guard =self.round_notification.write().expect("coordinator cant notify new round");
                *round_guard = this_round;
            }

            for player in players.iter()  {
                let (id, card) = player.get_card();
                println!("receiving card: {} from player {}",card, id);

            }


        }


        for mut player in players{
            player.wait();
        }

        println!("game ends");
    }


}