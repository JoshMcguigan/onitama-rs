use std::ops::Index;

use piece::{Side, Piece};
use location::Point;
use turn::Move;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Board([[Option<Piece>; 5]; 5]);
impl Board {
    pub fn new() -> Board {
        Board([
            [
                Some(Piece::new(Side::WHITE, false)),
                Some(Piece::new(Side::WHITE, false)),
                Some(Piece::new(Side::WHITE, true)),
                Some(Piece::new(Side::WHITE, false)),
                Some(Piece::new(Side::WHITE, false))
            ],
            [None; 5], [None; 5], [None; 5],
            [
                Some(Piece::new(Side::BLACK, false)),
                Some(Piece::new(Side::BLACK, false)),
                Some(Piece::new(Side::BLACK, true)),
                Some(Piece::new(Side::BLACK, false)),
                Some(Piece::new(Side::BLACK, false))
            ]
        ])
    }

    pub fn temple_of(s: Side) -> Point {
        match s {
            Side::WHITE => Point::new(2, 0),
            Side::BLACK => Point::new(2, 4)
        }
    }

    pub fn allowed(&self, m: Move) -> bool {
        let source_ok = match self[m.source] {
            Some(p) => p.side == m.player,
            None => false
        };
        let target_ok = match self[m.target] {
            Some(p) => p.side != m.player,
            None => true
        };
        source_ok && target_ok
    }

    pub fn after(&self, m: Move) -> Board {
        assert!(self.allowed(m));
        let mut board = self.0.clone();
        board[m.target.y as usize][m.target.x as usize] = self[m.source];
        board[m.source.y as usize][m.source.x as usize] = None;
        Board(board)
    }

    pub fn is_either_king_dead(&self) -> bool {
        self.iter().filter_map(|x| x.1).map(|p| p.king as u8).fold(0, |a, b| a+b) < 2
    }

    pub fn is_either_temple_taken(&self) -> bool {
        self.iter().filter_map(|x| x.1.map(|y| (x.0, y))).any(|x| Board::temple_of(x.1.side.other()) == x.0)
    }

    pub fn is_game_over(&self) -> bool {
        self.is_either_king_dead() || self.is_either_temple_taken()
    }

    pub fn iter(&self) -> BoardIter {
        BoardIter::new(*self)
    }

    pub fn as_string(&self) -> String {
        self.iter().map(|x| match x.1 {
            None => '.',
            Some(p) => match (p.side, p.king) {
                (Side::WHITE, false) => 'w',
                (Side::WHITE, true)  => 'W',
                (Side::BLACK, false) => 'b',
                (Side::BLACK, true)  => 'B',
            }
        }).collect()
    }
}
impl Index<Point> for Board {
    type Output = Option<Piece>;
    fn index(&self, p: Point) -> &Option<Piece> {
        &self.0[p.y as usize][p.x as usize]
    }
}


pub struct BoardIter {
    board: Board,
    index: usize
}
impl BoardIter {
    fn new(board: Board) -> BoardIter {
        BoardIter {board: board, index: 0}
    }
}
impl Iterator for BoardIter {
    type Item = (Point, Option<Piece>);

    fn next(&mut self) -> Option<(Point, Option<Piece>)> {
        if self.index < 25 {
            let p = Point::new((self.index % 5) as u8, (self.index / 5) as u8);
            self.index += 1;
            Some((p, self.board[p]))
        } else {None}
    }
}
