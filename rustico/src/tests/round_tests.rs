
use crate::card::french_card::FrenchCard;
use crate::card::card_suit::card_suit::CardSuit;
use crate::players::player::Player;
use crate::card::card_number::card_number::CardNumber;
use std::sync::{mpsc, Barrier, Arc, RwLock, Mutex, Condvar};
use crate::game::round::{Round, get_random_type_round};
use crate::game::normal_round::NormalRound;
use crate::game::rustic_round::RusticRound;
use crate::players::coordinator::PlayerCard;


#[test]
fn random_round_type() {
    let round_type = get_random_type_round();
    assert!(round_type.get_name().to_string() == "RUSTIC".to_string() || round_type.get_name().to_string() == "NORMAL".to_string())
}

#[test]
fn normal_round_compute_score_one_winner() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::K) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let round = NormalRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score(hand, get_players());
    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 10);
    assert_eq!(players[2].get_points(), 0);
    assert_eq!(players[3].get_points(), 0);
}


#[test]
fn normal_round_compute_score_two_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let round = NormalRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score( hand, get_players());

    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 5);
    assert_eq!(players[2].get_points(), 5);
    assert_eq!(players[3].get_points(), 0);
}

#[test]
fn normal_round_compute_score_three_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });

    let round = NormalRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score(hand, get_players());

    assert_eq!(players[0].get_points(), 0);
    assert_eq!(players[1].get_points(), 3);
    assert_eq!(players[2].get_points(), 3);
    assert_eq!(players[3].get_points(), 3);
}

#[test]
fn normal_round_compute_score_four_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::A) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let round = NormalRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score(hand, get_players());
    assert_eq!(players[0].get_points(), 2);
    assert_eq!(players[1].get_points(), 2);
    assert_eq!(players[2].get_points(), 2);
    assert_eq!(players[3].get_points(), 2);
}


#[test]
fn rustic_round_compute_score_one_winner() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::K) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let round = RusticRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score(hand, get_players());
    assert_eq!(players[0].get_points(), 10);
    assert_eq!(players[1].get_points(), 1);
    assert_eq!(players[2].get_points(), -5);
    assert_eq!(players[3].get_points(), 0);
}


#[test]
fn rustic_round_compute_score_two_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::FIVE) });


    let round = RusticRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score( hand, get_players());
    assert_eq!(players[0].get_points(), 1);
    assert_eq!(players[1].get_points(), 5);
    assert_eq!(players[2].get_points(), 5);
    assert_eq!(players[3].get_points(), -5);
}

#[test]
fn rustic_round_compute_score_three_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });


    let round = RusticRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score( hand, get_players());
    assert_eq!(players[0].get_points(), 3);
    assert_eq!(players[1].get_points(), 3);
    assert_eq!(players[2].get_points(), 1);
    assert_eq!(players[3].get_points(), -2);
}

#[test]
fn rustic_round_compute_score_four_draw() {
    let mut hand: Vec<PlayerCard> = Vec::with_capacity(4);
    hand.push(PlayerCard { player_id: 3, card: FrenchCard::new(CardSuit::CLOVER, CardNumber::A) });
    hand.push(PlayerCard { player_id: 2, card: FrenchCard::new(CardSuit::PIKE, CardNumber::A) });
    hand.push(PlayerCard { player_id: 1, card: FrenchCard::new(CardSuit::DIAMOND, CardNumber::A) });
    hand.push(PlayerCard { player_id: 0, card: FrenchCard::new(CardSuit::HEART, CardNumber::A) });

    let round = RusticRound::new(Option::None, false);
    let players: Vec<Player> = round.compute_score( hand, get_players());
    assert_eq!(players[0].get_points(), -3);
    assert_eq!(players[1].get_points(), 2);
    assert_eq!(players[2].get_points(), 2);
    assert_eq!(players[3].get_points(), 3);
}

fn get_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::with_capacity(4);

    for player_id in 0..4 {
        let cards: Vec<FrenchCard> = Vec::new();
        let (card_sender, _card_receiver) = mpsc::channel::<Option<PlayerCard>>();
        let barrier = Arc::new(Barrier::new(5));
        let arc: Arc<RwLock<Box<dyn Round>>> = Arc::new(RwLock::new(get_random_type_round()));
        let turn = Arc::new((Mutex::new(false), Condvar::new()));
        let next_turn = turn.clone();
        let player: Player = Player::new(player_id, card_sender, cards, barrier, turn, next_turn , arc );
        players.push(player);
    }
    return players;
}