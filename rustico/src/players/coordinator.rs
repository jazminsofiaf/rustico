use crate::players::round_type::round_type::RoundType;
use rand::{Rng, thread_rng};
use crate::players::player::Player;
use crate::card::french_card::{get_card_dec, FrenchCard};
use rand::seq::SliceRandom;


pub struct Coordinator {
     amount_of_players: i32
}

impl Coordinator {
    pub fn new(amount_of_players: i32) ->  Coordinator {
        return Coordinator {
            amount_of_players,
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


    pub fn deal_cards(&self, cards : Vec<FrenchCard>) ->  Vec<Player>{

        let amount_of_cards_by_player = cards.len() / self.amount_of_players as usize;
        println!("coordinator deal {} cards for each player", amount_of_cards_by_player);
        let mut card_iter = cards.into_iter().peekable();

        let mut players: Vec<Player>= Vec::with_capacity(self.amount_of_players as usize);
            for i in 0..self.amount_of_players {
                let cards_for_player: Vec<FrenchCard> = card_iter.by_ref().take(amount_of_cards_by_player).collect();
                let player: Player = Player::new(cards_for_player);
                players.push(player);
            }
        let remaining_cards: Vec<FrenchCard> =  card_iter.by_ref().take(amount_of_cards_by_player).collect();
        println!("remaining cards  {:?} ", remaining_cards);
        return players;
    }



    pub fn play_game(&self){
        println!("let the games begin");

        let deck : Vec<FrenchCard> = self.shuffle_deck();
        let players: Vec<Player> = self.deal_cards(deck);

        for mut player in players{
            player.wait();
        }

        println!("game ends");
    }


}