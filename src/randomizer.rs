/// A rudimentary bag-randomizer for Tetris

extern crate rand;
use rand::{thread_rng, Rng};
use piecedefs::*;

pub struct Randomizer {
    bag: Vec<Piecedef>
}

impl Randomizer {
    pub fn new() -> Randomizer {
        let bag = new_bag();
        Randomizer {
            bag: bag
        }
    }

    /// Takes a new piece out of the bag
    pub fn next_piece(&mut self) -> Piecedef {
        if self.bag.len() == 0 {
            self.bag = new_bag();
        }
        self.bag.pop().unwrap()
    }
}

/// Creates a new permutated bag
fn new_bag() -> Vec<Piecedef> {
    let mut rng = thread_rng();
    let mut bag = vec!(T, L, O, S, Z, J, I);
    rng.shuffle(&mut bag.as_mut_slice());
    while [S.id, Z.id, O.id].contains(&bag[0].id) {
        // TGM Ace randomizer, for whatever reason
        rng.shuffle(&mut bag.as_mut_slice());
    }
    bag
}
