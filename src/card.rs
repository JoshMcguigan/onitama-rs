use std::collections::HashSet;

use piece::Side;
use location::Step;
use cards::{CARDS, STARTER};

pub type StepAlternatives = HashSet<Step>;
pub type CardId = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Card(CardId);
impl Card {
    pub fn new(id: CardId) -> Card{
        assert!(id < CARDS.len());
        Card(id)
    }
    pub fn steps(&self) -> StepAlternatives {
        CARDS[self.0].clone()
    }
    pub fn flipped_steps(&self) -> StepAlternatives {
        self.steps().iter().map(|step| step.flip()).collect()
    }
    pub fn starter(&self) -> Side {
        STARTER[self.0]
    }
    pub fn id(&self) -> CardId {
        self.0
    }
}
