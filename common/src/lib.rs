use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    SmallBlind,
    BigBlind,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    READY,
    WAITING,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
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

#[derive(Debug, Clone, Serialize)]
pub enum GameState {
    PreDeal,
    Blinds,
    PreFlop,
    Flop,
    Turn,
    River,
    Shutdown,
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub role: Option<Role>,
    pub hand: Vec<Card>,
    pub state: PlayerState,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct PlayerRank {
    pub id: String,
    pub rank: HandRank,
}

#[derive(Debug, Clone, Serialize)]
pub enum GameResult {
    Winner(PlayerRank),
    Tie(PlayerRank, PlayerRank),
}

#[derive(Debug, Clone)]
pub struct TableDto {
    pub id: String,
    pub state: GameState,
    pub players: Vec<Player>,
    pub card_on_table: Vec<Card>,
    pub result: Option<GameResult>,
}

impl Player {
    pub fn new(id: String) -> Self {
        Player {
            id: id,
            role: None,
            hand: vec![],
            state: PlayerState::WAITING,
        }
    }
}

impl PlayerRank {
    pub fn of(id: &str, rank: HandRank) -> Self {
        PlayerRank {
            id: id.to_string(),
            rank: rank.clone(),
        }
    }
}

impl TableDto {
    pub fn from(
        id: String,
        players: Vec<Player>,
        state: GameState,
        card_on_table:Vec<Card>,
        result: Option<GameResult>,
    ) -> Self {
        TableDto {
            id,
            state,
            players,
            card_on_table,
            result,
        }
    }
}
