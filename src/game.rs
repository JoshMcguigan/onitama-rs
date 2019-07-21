use std::collections::HashSet;

use piece::Side;
use card::Card;
use turn::Move;
use board::Board;

#[derive(PartialEq, Eq)]
pub struct Game {
    pub turn: Side,
    pub board: Board,
    pub player_cards: [HashSet<Card>; 2],
    pub free_card: Card
}
impl Game {
    pub fn new(cards: [Card; 5]) -> Game {
        // Assertion fails during test right now
        // assert!({
        //     let mut cs: Vec<usize> = cards.clone().to_vec().iter().map(|c| c.id()).collect();
        //     cs.sort();
        //     cs.dedup();
        //     cs.len() == cards.len()
        // });

        Game {
            turn: cards[4].starter(),
            board: Board::new(),
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
        let ok = new_player_cards[self.turn.index()].remove(&m.card);
        assert!(ok);
        new_player_cards[self.turn.index()].insert(self.free_card);

        Game {
            turn: self.turn.other(),
            board: self.board.after(m),
            player_cards: new_player_cards,
            free_card: m.card
        }
    }
}
impl Clone for Game {
    fn clone(&self) -> Game {
        Game {
            turn: self.turn,
            board: self.board,
            player_cards: [self.player_cards[0].clone(), self.player_cards[1].clone()],
            free_card: self.free_card
        }
    }
}
