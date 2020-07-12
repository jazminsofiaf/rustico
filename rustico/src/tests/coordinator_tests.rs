use crate::card::french_card::{FrenchCard, get_card_deck};
use crate::players::coordinator::Coordinator;

#[test]
fn coordinator_shuffle_deck() {
    let card_deck: Vec<FrenchCard> = get_card_deck();
    let coordinator: Coordinator = Coordinator::new(5);
    assert_ne!(card_deck, coordinator.shuffle_deck());
}




