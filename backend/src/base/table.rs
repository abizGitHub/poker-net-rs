use serde::{Deserialize, Serialize};

use crate::base::card::{Card, Deck, HandRank, evaluate_hand};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    id: String,
    role: Option<Role>,
    hand: Vec<Card>,
    state: PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    READY,
    WAITING,
}

impl Player {
    fn new(id: String) -> Self {
        Player {
            id: id,
            role: None,
            hand: vec![],
            state: PlayerState::WAITING,
        }
    }
}

impl Into<PlayerDto> for &Player {
    fn into(self) -> PlayerDto {
        PlayerDto {
            id: self.id.clone(),
            role: self.role.clone(),
            state: self.state.clone(),
            hand: self.hand.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    SmallBlind,
    BigBlind,
    Others,
}

#[derive(Debug, Clone)]
struct Dealer {
    deck: Deck,
    game_state: GameState,
    players: Vec<Player>,
    cards_on_table: Vec<Card>,
    game_result: Option<GameResult>,
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

    fn evaluate_hands(&mut self) -> GameResult {
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

#[derive(Debug, Clone, Serialize)]
pub struct PlayerRank {
    pub id: String,
    pub rank: HandRank,
}

impl PlayerRank {
    pub fn of(id: &str, rank: HandRank) -> Self {
        PlayerRank {
            id: id.to_string(),
            rank: rank.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GameResult {
    Winner(PlayerRank),
    Tie(PlayerRank, PlayerRank),
}

#[derive(Debug, Clone)]
pub struct GameTable {
    dealer: Dealer,
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

    pub fn players(&self) -> Vec<PlayerDto> {
        self.dealer.players.iter().map(|p| p.into()).collect()
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
pub struct PlayerDto {
    pub id: String,
    pub role: Option<Role>,
    pub state: PlayerState,
    pub hand: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct TableDto {
    pub id: String,
    pub state: GameState,
    pub players: Vec<PlayerDto>,
    pub card_on_table: Vec<Card>,
    pub result: Option<GameResult>,
}

impl TableDto {
    pub fn from(table_id: &str, table: &GameTable, result: Option<GameResult>) -> Self {
        TableDto {
            id: table_id.to_string(),
            players: table.players(),
            state: table.dealer.game_state.clone(),
            card_on_table: table.dealer.cards_on_table.clone(),
            result,
        }
    }
}
