#![allow(dead_code)]


#[macro_use]
extern crate lazy_static;

pub mod location;
pub mod piece;
pub mod card;
pub mod cards;
pub mod turn;
pub mod board;
pub mod game;

#[cfg(test)]
mod tests {
    use game::Game;
    use card::*;
    use turn::*;
    use location::*;

    #[test]
    fn test_short_game() {
        fn take_first_card(g: &Game) -> Card {
            *g.current_player_cards().iter().take(1).last().unwrap()
        }

        let mut g = Game::new([Card::new(4), Card::new(10), Card::new(10), Card::new(12), Card::new(13)]);
        assert!(!g.is_over());

        let fc = take_first_card(&g);
        g = g.after(Move::new(g.turn, Point::new(2, 4), Point::new(2, 3), fc));
        assert!(!g.is_over());

        let fc = take_first_card(&g);
        g = g.after(Move::new(g.turn, Point::new(2, 0), Point::new(2, 1), fc));
        assert!(!g.is_over());

        let fc = take_first_card(&g);
        g = g.after(Move::new(g.turn, Point::new(2, 3), Point::new(2, 2), fc));
        assert!(!g.is_over());

        let fc = take_first_card(&g);
        g = g.after(Move::new(g.turn, Point::new(2, 1), Point::new(2, 2), fc));
        assert!(g.is_over());

    }
}
