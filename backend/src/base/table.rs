use serde::{Deserialize, Serialize};

use crate::base::card::{Card, Deck};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Role {
    SmallBlind,
    BigBlind,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    fn remove_player(&mut self, p_id: &str) {
        if let Some(pos) = self.players.iter().position(|p| p.id == p_id) {
            self.players.remove(pos);
        } else {
            println!("Couldn't find pl with the specified id");
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTable {
    dealer: Dealer,
}

impl GameTable {
    pub fn set_a_table() -> Self {
        GameTable {
            dealer: Dealer::new(vec![]),
        }
    }

    pub fn add_player(&mut self, id: &str) {
        self.dealer.players.push(Player::new(id.to_string()));
    }

    pub fn players(&self) -> Vec<String> {
        self.dealer.players.iter().map(|p| p.id.clone()).collect()
    }

    pub fn remove_player(&mut self, id: &str) {
        self.dealer.remove_player(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum GameState {
    PreDeal,
    Blinds,
    PreFlop,
    Flop,
    Turn,
    River,
    Shutdown,
}
