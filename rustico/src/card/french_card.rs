use core::fmt;
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;
use std::cmp::Ordering;


#[derive(PartialEq, Eq, Debug)]
pub struct FrenchCard {
    suit: CardSuit,
    number: CardNumber,
}

impl FrenchCard {
    pub fn new(suit: CardSuit, number: CardNumber) -> FrenchCard {
        FrenchCard { suit, number }
    }
}

impl Ord for FrenchCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for FrenchCard {
    fn partial_cmp(&self, other: &FrenchCard) -> Option<Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

impl fmt::Display for FrenchCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.suit, self.number)
    }
}

pub fn get_card_dec() -> Vec<FrenchCard> {
    let mut card_dec: Vec<FrenchCard> = Vec::new();
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TWO));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::THREE));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::FOUR));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::FIVE));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::SIX));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::SEVEN));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::EIGHT));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::NINE));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::J));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::Q));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::K));
    card_dec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::A));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::TWO));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::THREE));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::FOUR));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::FIVE));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::SIX));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::SEVEN));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::EIGHT));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::NINE));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::TEN));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::J));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::Q));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::K));
    card_dec.push(FrenchCard::new(CardSuit::HEART, CardNumber::A));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::TWO));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::THREE));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::FOUR));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::FIVE));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::SIX));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::SEVEN));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::EIGHT));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::NINE));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::TEN));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::J));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::Q));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::K));
    card_dec.push(FrenchCard::new(CardSuit::PIKE, CardNumber::A));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::TWO));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::THREE));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::FOUR));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::FIVE));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::SIX));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::SEVEN));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::EIGHT));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::NINE));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::TEN));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::J));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::Q));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::K));
    card_dec.push(FrenchCard::new(CardSuit::DIAMOND, CardNumber::A));
    return card_dec;
}


