use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rank {
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Card {
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

// Poker hand ranks
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HandRank {
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
