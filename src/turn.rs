use piece::Side;
use card::Card;
use location::Point;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub player: Side,
    pub source: Point,
    pub target: Point,
    pub card: Card
}
impl Move {
    pub fn new(p: Side, s: Point, t: Point, c: Card) -> Move {
        Move {player: p, source: s, target: t, card: c}
    }
}
