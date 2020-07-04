use crate::players::coordinator::Coordinator;

#[test]
fn coordinator_decide_round_type() {
    let coordinator: Coordinator = Coordinator::new();
    let round_type = coordinator.get_round_type();
    assert!(round_type.to_string() == "RUSTIC".to_string() || round_type.to_string() == "NORMAL".to_string()  )
    
}

