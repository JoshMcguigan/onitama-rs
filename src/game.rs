use std::collections::HashSet;

use piece::Side;
use card::Card;
use turn::Move;
use board::Board;

#[derive(PartialEq, Eq)]
pub struct Game {
    pub turn: Side,
    board: Board,
    start_cards: [Card; 5],
    player_cards: [HashSet<Card>; 2],
    free_card: Card
}
impl Game {
    pub fn new(cards: [Card; 5]) -> Game {
        Game {
            turn: cards[4].starter(),
            board: Board::new(),
            start_cards: cards,
            player_cards: [
                {
                    let mut hs = HashSet::new();
                    hs.insert(cards[0]);
                    hs.insert(cards[1]);
                    hs
                },
                {
                    let mut hs = HashSet::new();
                    hs.insert(cards[2]);
                    hs.insert(cards[3]);
                    hs
                }
            ],
            free_card: cards[4]
        }
    }

    pub fn cards_of(&self, side: Side) -> HashSet<Card> {
        self.player_cards[side.index()].clone()
    }

    pub fn current_player_cards(&self) -> HashSet<Card> {
        self.cards_of(self.turn)
    }

    pub fn is_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn after(&self, m: Move) -> Game {
        assert!(m.player == self.turn);

        let mut new_player_cards = [self.player_cards[0].clone(), self.player_cards[1].clone()];
        new_player_cards[self.turn.index()].remove(&m.card);
        new_player_cards[self.turn.index()].insert(self.free_card);

        Game {
            turn: self.turn.other(),
            board: self.board.after(m),
            start_cards: self.start_cards,
            player_cards: new_player_cards,
            free_card: m.card
        }
    }
}
