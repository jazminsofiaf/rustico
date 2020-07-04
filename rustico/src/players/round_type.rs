
pub mod round_type {
    use core::fmt;
    use rand::Rng;
    use rand::distributions::{Distribution, Standard};


    #[derive(PartialEq, Eq)]
    pub enum RoundType {
        NORMAL,
        RUSTIC
    }

    impl RoundType{
        fn value(&self)-> String {
            match *self {
                RoundType::NORMAL => "NORMAL",
                RoundType::RUSTIC => "RUSTIC",
            }.to_string()
        }

    }

    impl Distribution<RoundType>  for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RoundType {
            match rng.gen_range(0, 2) {
                0 => RoundType::NORMAL,
                _ => RoundType::RUSTIC,
            }
        }
    }

    impl fmt::Display for RoundType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value())
        }
    }
}