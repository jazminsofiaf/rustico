pub mod card_suit {
    use core::fmt;

    #[derive(PartialEq, Eq, Debug)]
    pub enum CardSuit {
        CLOVER,
        HEART,
        DIAMOND,
        PIKE,
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
}