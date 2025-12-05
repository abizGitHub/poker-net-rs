use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{self};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        let suit = match self.suit {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };

        write!(f, "{}{}", rank, suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
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

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

// Poker hand ranks
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Rank),
    Pair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourOfKind(Rank),
    StraightFlush(Rank),
}

fn evaluate_hand(cards: &[Card]) -> HandRank {
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

struct Player {
    id: String,
    role: Option<Role>,
    hand: Vec<Card>,
}

impl Player {
    fn new(id: String) -> Self {
        Player {
            id: id,
            role: None,
            hand: vec![],
        }
    }
}

enum Role {
    SmallBlind,
    BigBlind,
    Others,
}
struct Dealer {
    deck: Deck,
    game_state: GameState,
    players: Vec<Player>,
    cards_on_table: Vec<Card>,
}

impl Dealer {
    fn new(players: Vec<Player>) -> Self {
        Dealer {
            deck: Deck::new(),
            game_state: GameState::PreDeal,
            players,
            cards_on_table: Vec::new(),
        }
    }

    fn knock_knock(&mut self) {
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
                self.players
                    .iter_mut()
                    .filter(|p| p.role.is_some())
                    .for_each(|p| p.hand.push(self.deck.deal()));
            }
            GameState::River => {
                self.players
                    .iter_mut()
                    .filter(|p| p.role.is_some())
                    .for_each(|p| p.hand.push(self.deck.deal()));
            }
            GameState::Shutdown => {}
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
            GameState::Shutdown => GameState::Shutdown,
        };
    }
}

pub struct GameTable {
    dealer: Dealer,
}

impl GameTable {
    pub fn set_a_table() -> Self {
        GameTable {
            dealer: Dealer::new(vec![]),
        }
    }

    fn add_player(&mut self, id: String) {
        self.dealer.players.push(Player::new(id));
    }
}

enum GameState {
    PreDeal,
    Blinds,
    PreFlop,
    Flop,
    Turn,
    River,
    Shutdown,
}

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
