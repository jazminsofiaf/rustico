use crate::card::french_card::FrenchCard;
use crate::card::card_suit::card_suit::CardSuit;
use crate::card::card_number::card_number::CardNumber;


#[test]
fn card_equality() {
    assert_ne!(FrenchCard::new(CardSuit::CLOVER, CardNumber::K), FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN));
    assert_ne!(FrenchCard::new(CardSuit::CLOVER, CardNumber::K), FrenchCard::new(CardSuit::DIAMOND, CardNumber::K));
    assert_eq!(FrenchCard::new(CardSuit::CLOVER, CardNumber::K), FrenchCard::new(CardSuit::CLOVER, CardNumber::K));
}

#[test]
fn card_cmp() {
    let card_clover = FrenchCard::new(CardSuit::CLOVER, CardNumber::TWO);
    let card_pike = FrenchCard::new(CardSuit::PIKE, CardNumber::THREE);
    assert_eq!(card_clover < card_pike, true);

    let card_clover = FrenchCard::new(CardSuit::CLOVER, CardNumber::J);
    let card_pike = FrenchCard::new(CardSuit::PIKE, CardNumber::K);
    assert_eq!(card_clover < card_pike, true);
}

#[test]
fn get_max_card_only_numbers() {
    let mut cards_vec: Vec<FrenchCard> = Vec::new();
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TWO));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::THREE));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::FOUR));
    assert_eq!(cards_vec.iter().max().unwrap(), &FrenchCard::new(CardSuit::CLOVER, CardNumber::FOUR));
}

#[test]
fn get_max_card_letter() {
    let mut cards_vec: Vec<FrenchCard> = Vec::new();
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::TEN));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::K));
    cards_vec.push(FrenchCard::new(CardSuit::CLOVER, CardNumber::J));
    assert_eq!(cards_vec.iter().max().unwrap(), &FrenchCard::new(CardSuit::CLOVER, CardNumber::K));
}