#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    WHITE, BLACK
}
impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::WHITE => Side::BLACK,
            Side::BLACK => Side::WHITE
        }
    }
    pub fn index(self) -> usize {
        match self {
            Side::WHITE => 0,
            Side::BLACK => 1
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Piece {
    pub side: Side,
    pub king: bool
}
impl Piece {
    pub fn new(side: Side, king: bool) -> Piece {
        Piece {side: side, king: king}
    }
}
