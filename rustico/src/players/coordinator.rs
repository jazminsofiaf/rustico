use crate::players::round_type::round_type::RoundType;
use rand::Rng;


pub struct Coordinator {}

impl Coordinator {
    pub fn new() ->  Coordinator {
        return Coordinator {};
    }

    pub fn get_round_type(&self) -> RoundType {
        let mut rng = rand::thread_rng();
        let round_type: RoundType = rng.gen();
        return  round_type;
    }


}