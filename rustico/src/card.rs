use core::fmt;

#[derive(PartialEq, Eq)]
pub enum CardSuit  {
    CLOVER, HEART, DIAMOND, PIKE,
}
impl CardSuit {
    fn value(&self) -> char {
        match *self {
            CardSuit::CLOVER => '♣',
            CardSuit::HEART => '❤',
            CardSuit::DIAMOND => '♦',
            CardSuit::PIKE => '♠',
        }
    }
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
// call like CardSuit::enumvalue.value()

#[derive(PartialEq, Eq)]
pub enum CardNumber  {
    TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN,
    J, Q, K, A
}

impl fmt::Display for CardNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl CardNumber {
    fn value(&self) -> u32 {
        match *self {
            CardNumber::TWO => 2,
            CardNumber::THREE => 3,
            CardNumber::FOUR => 4,
            CardNumber::FIVE => 5,
            CardNumber::SIX => 6,
            CardNumber::SEVEN => 7,
            CardNumber::EIGHT => 8,
            CardNumber::NINE => 9,
            CardNumber::TEN => 10,
            CardNumber::J => 11,
            CardNumber::Q => 12,
            CardNumber::K => 13,
            CardNumber::A => 14,
        }
    }
}


#[derive(PartialEq, Eq)]
pub struct FrenchCard {
    suit: CardSuit,
    number: CardNumber,
}

impl  FrenchCard {

    pub(crate) fn new(suit: CardSuit, number: CardNumber) ->  FrenchCard {
        FrenchCard { suit, number }
    }
}

impl fmt::Display for FrenchCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.suit, self.number)
    }
}

pub fn get_card_dec() ->  Vec<FrenchCard> {
    let mut card_dec: Vec<FrenchCard> = Vec::new();
    card_dec.push(FrenchCard::new(CardSuit::CLOVER,  CardNumber::TWO ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::THREE ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::FOUR ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::FIVE ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::SIX ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::SEVEN ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::EIGHT ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::NINE ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::TEN ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::J ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::Q ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::K ));
    card_dec.push(FrenchCard::new( CardSuit::CLOVER, CardNumber::A ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::TWO ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::THREE ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::FOUR ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::FIVE ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::SIX ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::SEVEN ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::EIGHT ));
    card_dec.push( FrenchCard::new( CardSuit::HEART, CardNumber::NINE ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::TEN ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::J ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::Q ));
    card_dec.push(FrenchCard::new( CardSuit::HEART, CardNumber::K ));
    card_dec.push( FrenchCard::new( CardSuit::HEART, CardNumber::A ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::TWO ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::THREE ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::FOUR ));
    card_dec.push( FrenchCard::new( CardSuit::PIKE, CardNumber::FIVE ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::SIX ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::SEVEN ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::EIGHT ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::NINE ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::TEN ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::J ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::Q ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::K ));
    card_dec.push(FrenchCard::new( CardSuit::PIKE, CardNumber::A ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::TWO ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::THREE ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::FOUR ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::FIVE ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::SIX ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::SEVEN ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::EIGHT ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::NINE ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::TEN ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::J ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::Q ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::K ));
    card_dec.push(FrenchCard::new( CardSuit::DIAMOND, CardNumber::A ));
    return card_dec;
}


