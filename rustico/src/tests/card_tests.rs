use crate::card::french_card::FrenchCard;
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;

#[test]
fn get_max_card_only_numbers() {
    let mut cards_vec: Vec<FrenchCard> = Vec::new();
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TWO ));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::THREE ));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::FOUR ));
    assert_eq!(cards_vec.iter().max().unwrap(), &FrenchCard::new(CardSuit::CLOVER, CardNumber::FOUR));
}

#[test]
fn get_max_card_letter() {
    let mut cards_vec: Vec<FrenchCard> = Vec::new();
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN ));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::K ));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::J ));
    assert_eq!(cards_vec.iter().max().unwrap(), &FrenchCard::new(CardSuit::CLOVER, CardNumber::K));
}