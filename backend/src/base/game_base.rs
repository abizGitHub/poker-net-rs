use rand::seq::SliceRandom;
use rand::thread_rng;
use common::*;

#[derive(Debug, Clone)]
pub struct Dealer {
    deck: Deck,
    pub game_state: GameState,
    players: Vec<Player>,
    pub cards_on_table: Vec<Card>,
    game_result: Option<GameResult>,
}

#[derive(Debug, Clone)]
pub struct GameTable {
    pub dealer: Dealer,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &s in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for &r in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                cards.push(Card { rank: r, suit: s });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

impl GameTable {
    pub fn set_a_table() -> Self {
        let mut table = GameTable {
            dealer: Dealer::new(vec![]),
        };
        table.dealer.knock_knock();
        table
    }

    pub fn add_player(&mut self, id: &str) {
        self.dealer.players.push(Player::new(id.to_string()));
    }

    pub fn players(&self) -> Vec<Player> {
        self.dealer.players.iter().map(|p| p.clone()).collect()
    }

    pub fn remove_player(&mut self, id: &str) {
        self.dealer.remove_player(id)
    }

    pub fn player_change_state(&mut self, id: &str, state: &PlayerState) -> bool {
        self.dealer.player_change_state(id, state)
    }

    pub fn get_result(&self) -> Option<GameResult> {
        self.dealer.game_result.clone()
    }
}

impl Dealer {
    fn new(players: Vec<Player>) -> Self {
        Dealer {
            deck: Deck::new(),
            game_state: GameState::PreDeal,
            players,
            cards_on_table: Vec::new(),
            game_result: None,
        }
    }

    fn remove_player(&mut self, p_id: &str) {
        if let Some(pos) = self.players.iter().position(|p| p.id == p_id) {
            self.players.remove(pos);
        } else {
            println!("Couldn't find pl with the specified id");
        }
    }

    pub fn player_change_state(&mut self, p_id: &str, state: &PlayerState) -> bool {
        if let Some(pos) = self.players.iter().position(|p| p.id == p_id) {
            self.players.get_mut(pos).unwrap().state = state.clone();
            if self.players.iter().all(|p| p.state == PlayerState::READY) {
                self.knock_knock();
                self.players
                    .iter_mut()
                    .for_each(|p| p.state = PlayerState::WAITING);

                true
            } else {
                false
            }
        } else {
            println!("Couldn't find pl with the specified id");
            false
        }
    }

    pub fn knock_knock(&mut self) {
        match self.game_state {
            GameState::PreDeal => self.deck.shuffle(),

            GameState::Blinds => {
                self.players[0].role = Some(Role::SmallBlind);
                self.players[1].role = Some(Role::BigBlind);
                self.players
                    .iter_mut()
                    .skip(2)
                    .for_each(|p| p.role = Some(Role::Others));
            }

            GameState::PreFlop => {
                self.players
                    .iter_mut()
                    .for_each(|p| p.hand = vec![self.deck.deal(), self.deck.deal()]);
            }

            GameState::Flop => {
                self.cards_on_table = vec![self.deck.deal(), self.deck.deal(), self.deck.deal()]
            }

            GameState::Turn => {
                self.cards_on_table.push(self.deck.deal());

                self.players
                    .iter_mut()
                    .filter(|p| p.role.is_some())
                    .for_each(|p| p.hand.push(self.deck.deal()));
            }
            GameState::River => {
                self.cards_on_table.push(self.deck.deal());

                self.players
                    .iter_mut()
                    .filter(|p| p.role.is_some())
                    .for_each(|p| p.hand.push(self.deck.deal()));
            }
            GameState::Shutdown => {
                self.game_result = Some(self.evaluate_hands());
            }
            GameState::Ended => {}
        };
        self.change_state();
    }

    fn change_state(&mut self) {
        self.game_state = match self.game_state {
            GameState::PreDeal => GameState::Blinds,
            GameState::Blinds => GameState::PreFlop,
            GameState::PreFlop => GameState::Flop,
            GameState::Flop => GameState::Turn,
            GameState::Turn => GameState::River,
            GameState::River => GameState::Shutdown,
            GameState::Shutdown => GameState::Ended,
            GameState::Ended => GameState::Ended,
        };
    }

    fn evaluate_hands(&self) -> GameResult {
        let mut ranks: Vec<PlayerRank> = self
            .players
            .iter()
            .map(|p| {
                PlayerRank::of(
                    &p.id,
                    evaluate_hand(&[p.hand.clone(), self.cards_on_table.clone()].concat()),
                )
            })
            .collect();

        ranks.sort_by(|a, b| b.rank.cmp(&a.rank));
        let first = ranks.get(0).unwrap();
        let second = ranks.get(1).unwrap();

        match first.rank.cmp(&second.rank) {
            std::cmp::Ordering::Greater => GameResult::Winner(first.clone()),
            std::cmp::Ordering::Less => panic!("Impossible ranking sort!"),
            std::cmp::Ordering::Equal => GameResult::Tie(first.clone(), second.clone()),
        }
    }
}

pub fn evaluate_hand(cards: &[Card]) -> HandRank {
    use HandRank::*;

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();

    let mut counts = std::collections::HashMap::new();
    for &r in &ranks {
        *counts.entry(r).or_insert(0) += 1;
    }

    let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);

    let is_straight = ranks.windows(2).all(|w| (w[1] as u8) == (w[0] as u8 + 1))
        || (ranks == vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace]); // A-5 straight

    let highest = *ranks.last().unwrap();

    if is_straight && is_flush {
        return StraightFlush(highest);
    }

    for (&rank, &count) in &counts {
        if count == 4 {
            return FourOfKind(rank);
        }
    }

    if counts.values().any(|&c| c == 3) && counts.values().any(|&c| c == 2) {
        let three = *counts.iter().find(|&(_, &c)| c == 3).unwrap().0;
        let two = *counts.iter().find(|&(_, &c)| c == 2).unwrap().0;
        return FullHouse(three, two);
    }

    if is_flush {
        return Flush(highest);
    }

    if is_straight {
        return Straight(highest);
    }

    if let Some((&rank, _)) = counts.iter().find(|&(_, &c)| c == 3) {
        return ThreeOfKind(rank);
    }

    let pairs: Vec<Rank> = counts
        .iter()
        .filter(|&(_, &c)| c == 2)
        .map(|(&r, _)| r)
        .collect();

    if pairs.len() == 2 {
        let mut p = pairs.clone();
        p.sort();
        return TwoPair(p[1], p[0]);
    }

    if pairs.len() == 1 {
        return Pair(pairs[0]);
    }

    HighCard(highest)
}

#[allow(dead_code)]
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
