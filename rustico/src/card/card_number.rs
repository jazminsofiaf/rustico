pub mod card_number {
    use core::fmt;

    #[derive(PartialEq, Eq)]
    pub enum CardNumber {
        TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
        EIGHT,
        NINE,
        TEN,
        J,
        Q,
        K,
        A
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
}