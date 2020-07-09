use crate::players::coordinator::{Coordinator, PlayerCard};
use crate::card::french_card::{FrenchCard, get_card_dec};
use crate::players::player::Player;
use std::sync::{Arc, Mutex, Condvar, mpsc, RwLock, Barrier};
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;
use crate::players::round_type::round_type::RoundType::{NORMAL, RUSTIC};
use crate::game::round::Round;

#[test]
fn coordinator_decide_round_type() {
    let coordinator: Coordinator = Coordinator::new(5);
    let round_type = coordinator.get_round_type();
    assert!(round_type.to_string() == "RUSTIC".to_string() || round_type.to_string() == "NORMAL".to_string())
}

#[test]
fn coordinator_shuffle_deck() {
    let card_deck: Vec<FrenchCard> = get_card_dec();
    let coordinator: Coordinator = Coordinator::new(5);
    assert_ne!(card_deck, coordinator.shuffle_deck());
}

#[test]
fn coordinator_compute_score_one_winner() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::K) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let coordinator: Coordinator = Coordinator::new( 4);
    let players: Vec<Player> = coordinator.compute_score(NORMAL, hand, get_players());
    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 10);
    assert_eq!(players[2].get_points(), 0);
    assert_eq!(players[3].get_points(), 0);
}


#[test]
fn coordinator_compute_score_two_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let coordinator: Coordinator = Coordinator::new(4);
    let players: Vec<Player> = coordinator.compute_score(NORMAL,hand, get_players());
    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 5);
    assert_eq!(players[2].get_points(), 5);
    assert_eq!(players[3].get_points(), 0);
}

#[test]
fn coordinator_compute_score_three_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let coordinator: Coordinator = Coordinator::new(4);
    let players: Vec<Player> = coordinator.compute_score(NORMAL,hand, get_players());
    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 3);
    assert_eq!(players[2].get_points(), 3);
    assert_eq!(players[3].get_points(), 3);
}

#[test]
fn coordinator_compute_score_four_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::A) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let coordinator: Coordinator = Coordinator::new(4);
    let players: Vec<Player> = coordinator.compute_score(NORMAL,hand, get_players());
    assert_eq!(players[0].get_points(), 2);
    assert_eq!(players[1].get_points(), 2);
    assert_eq!(players[2].get_points(), 2);
    assert_eq!(players[3].get_points(), 2);
}


#[test]
fn coordinator_compute_score_one_winner_rustic() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::K) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let coordinator: Coordinator = Coordinator::new( 4);
    let players: Vec<Player> = coordinator.compute_score(RUSTIC, hand, get_players());
    assert_eq!(players[0].get_points(), 10);
    assert_eq!(players[1].get_points(), 1);
    assert_eq!(players[2].get_points(), -5);
    assert_eq!(players[3].get_points(), 0);
}


#[test]
fn coordinator_compute_score_two_draw_rustic() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let coordinator: Coordinator = Coordinator::new(4);
    let players: Vec<Player> = coordinator.compute_score(RUSTIC, hand, get_players());
    assert_eq!(players[0].get_points(), 1);
    assert_eq!(players[1].get_points(), 5);
    assert_eq!(players[2].get_points(), 5);
    assert_eq!(players[3].get_points(), -5);
}

#[test]
fn coordinator_compute_score_three_draw_rustic() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let coordinator: Coordinator = Coordinator::new(4);
    let players: Vec<Player> = coordinator.compute_score(RUSTIC, hand, get_players());
    assert_eq!(players[0].get_points(), 3);
    assert_eq!(players[1].get_points(), 3);
    assert_eq!(players[2].get_points(), 1);
    assert_eq!(players[3].get_points(), -2);
}

#[test]
fn coordinator_compute_score_four_draw_rustic() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let coordinator: Coordinator = Coordinator::new(4);
    let round = Round::new(Option::None,  false);
    let round_info: Arc<RwLock<Round>> = Arc::new(RwLock::new(round));
    let players: Vec<Player> = coordinator.compute_score(RUSTIC, hand, get_players());
    assert_eq!(players[0].get_points(), -3);
    assert_eq!(players[1].get_points(), 2);
    assert_eq!(players[2].get_points(), 2);
    assert_eq!(players[3].get_points(), 3);
}

fn get_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::with_capacity(4);
    for player_id in 0..4 {
        let cards: Vec<FrenchCard> = Vec::new();
        let (card_sender, card_receiver) = mpsc::channel::<PlayerCard>();
        let barrier = Arc::new(Barrier::new(5));
        let round = Round::new(Option::None,  false);
        let arc: Arc<RwLock<Round>> = Arc::new(RwLock::new(round));
        let player: Player = Player::new(player_id, card_sender, cards, barrier, arc );
        players.push(player);
    }
    return players;
}
