use crate::players::coordinator::Coordinator;
use crate::card::french_card::{FrenchCard, get_card_dec};

#[test]
fn coordinator_decide_round_type() {
    let coordinator: Coordinator = Coordinator::new(5);
    let round_type = coordinator.get_round_type();
    assert!(round_type.to_string() == "RUSTIC".to_string() || round_type.to_string() == "NORMAL".to_string()  )

}

#[test]
fn coordinator_shuffle_deck() {
    let  card_deck: Vec<FrenchCard> = get_card_dec();
    let coordinator: Coordinator = Coordinator::new(5);
    assert_ne!(card_deck, coordinator.shuffle_deck());

}



