use crate::base::card::{Deck, evaluate_hand};


// temp : for test
pub fn tmp_two_player() -> Vec<String> {
    let mut out = Vec::new();
    out.push(" ".to_string());
    out.push("===========================".to_string());
    let mut deck = Deck::new();
    deck.shuffle();

    // Deal 2 players
    let player1 = vec![deck.deal(), deck.deal()];
    let player2 = vec![deck.deal(), deck.deal()];

    // Community cards
    let table = vec![
        deck.deal(),
        deck.deal(),
        deck.deal(),
        deck.deal(),
        deck.deal(),
    ];

    out.push(format!("Player 1: {} {}", player1[0], player1[1]));
    out.push(format!("Player 2: {} {}", player1[0], player1[1]));
    out.push(format!(
        "Table: {:?}",
        table.iter().map(|c| c.to_string()).collect::<Vec<_>>()
    ));

    let p1_best = evaluate_hand(&[player1.clone(), table.clone()].concat());
    let p2_best = evaluate_hand(&[player2.clone(), table.clone()].concat());

    out.push(format!("P1 hand: {:?}", p1_best));
    out.push(format!("P2 hand: {:?}", p2_best));

    match p1_best.cmp(&p2_best) {
        std::cmp::Ordering::Greater => out.push(format!("Player 1 wins!")),
        std::cmp::Ordering::Less => out.push(format!("Player 2 wins!")),
        std::cmp::Ordering::Equal => out.push(format!("It's a tie!")),
    }
    out.push("===========================".to_string());
    out
}
