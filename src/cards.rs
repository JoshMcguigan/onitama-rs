use std::collections::HashSet;

use piece::Side;
use location::Step;
use card::StepAlternatives;

macro_rules! s {
    ($dx:expr, $dy:expr) => {
        Step::new($dx, $dy)
    };
}

macro_rules! c {
    ($($x:expr),*) => {
        {
            let mut hs = HashSet::new();
            $( hs.insert($x); )*
            hs
        }
    };
}

// https://1.bp.blogspot.com/-dm4YAFl2KVs/V3whsIEs65I/AAAAAAAADqQ/doS23VHY794QBcKyDmhZHXE5oejh9LgkwCKgB/s1600/Onitama%2B3.jpg

lazy_static! {
    pub static ref CARDS: [StepAlternatives; 16] = [
        c!(s!(0, -2), s!(0, 1)),                        // tiger
        c!(s!(-2, -1), s!(2, -1), s!(-1, 1), s!(1, 1)), // dragon
        c!(s!(-2, 0), s!(-1, -1), s!(1, 1)),            // frog
        c!(s!(2, 0), s!(1, -1), s!(-1, 1)),             // rabbit
        c!(s!(0, -1), s!(-2, 0), s!(2, 0)),             // crab
        c!(s!(-1, 0), s!(-1, -1), s!(1, 0), s!(1, -1)), // elephant
        c!(s!(-1, 0), s!(-1, -1), s!(1, 0), s!(1, 1)),  // goose
        c!(s!(-1, 0), s!(-1, 1), s!(1, 0), s!(1, -1)),  // rooster
        c!(s!(-1, -1), s!(-1, 1), s!(1, -1), s!(1, 1)), // monkey
        c!(s!(0, 1), s!(-1, -1), s!(1, -1)),            // mantis
        c!(s!(0, -1), s!(0, 1), s!(-1, 0)),             // horse
        c!(s!(0, -1), s!(0, 1), s!(1, 0)),              // ox
        c!(s!(0, -1), s!(-1, 1), s!(1, 1)),             // crane
        c!(s!(0, -1), s!(-1, 0), s!(1, 0)),             // boar
        c!(s!(-1, -1), s!(-1, 1), s!(1, 0)),            // eel
        c!(s!(1, -1), s!(1, 1), s!(-1, 0)),             // cobra
    ];
}

pub static STARTER: [Side; 16] = [
    Side::WHITE, Side::BLACK, Side::BLACK, Side::WHITE,
    Side::WHITE, Side::BLACK, Side::WHITE, Side::BLACK,
    Side::WHITE, Side::BLACK, Side::BLACK, Side::WHITE,
    Side::WHITE, Side::BLACK, Side::WHITE, Side::BLACK
];

pub static CARD_NAMES: [&'static str; 16] = [
    "tiger",
    "dragon",
    "frog",
    "rabbit",
    "crab",
    "elephant",
    "goose",
    "rooster",
    "monkey",
    "mantis",
    "horse",
    "ox",
    "crane",
    "boar",
    "eel",
    "cobra"
];
