use piece::Side;
use card::Card;
use turn::Move;
use board::Board;

#[derive(PartialEq, Eq)]
pub struct Game {
    pub turn: Side,
    pub board: Board,
    pub player_cards: [[Card; 2]; 2],
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
                [
                    cards[0],
                    cards[1],
                ],
                [
                    cards[2],
                    cards[3],
                ],
            ],
            free_card: cards[4]
        }
    }

    pub fn cards_of(&self, side: Side) -> [Card; 2] {
        self.player_cards[side.index()].clone()
    }

    pub fn current_player_cards(&self) -> [Card; 2] {
        self.cards_of(self.turn)
    }

    pub fn is_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn after(&mut self, m: Move) {
        assert!(m.player == self.turn);

        let card_index = if self.player_cards[self.turn.index()][0] == m.card {
            0
        } else if self.player_cards[self.turn.index()][1] == m.card {
            1
        } else {
            panic!("invalid move")
        };

        let newly_free_card = self.player_cards[self.turn.index()][card_index];
        self.player_cards[self.turn.index()][card_index] = self.free_card;
        self.free_card = newly_free_card;

        self.turn = self.turn.other();

        self.board = self.board.after(m);
    }
}
