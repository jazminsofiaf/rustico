use crate::card::french_card::{FrenchCard, get_card_deck};
use crate::players::coordinator::Coordinator;
use std::sync::mpsc;
use crate::logger::logger::Logger;

#[test]
fn coordinator_shuffle_deck() {
    let (logger_sender, logger_receiver) = mpsc::channel();
    let _logger = Logger::new(false, logger_receiver);

    let card_deck: Vec<FrenchCard> = get_card_deck();
    let coordinator: Coordinator = Coordinator::new(5, logger_sender.clone());
    assert_ne!(card_deck, coordinator.shuffle_deck());
}
