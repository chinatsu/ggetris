/// A rudimentary bag-randomizer for Tetris

extern crate rand;
use rand::{thread_rng, seq::SliceRandom};
use crate::piecedefs::*;
use std::collections::VecDeque;

pub struct Randomizer {
    history: VecDeque<Piecedef>
}

impl Randomizer {
    pub fn new() -> Randomizer {
        let mut history: VecDeque<Piecedef> = VecDeque::with_capacity(4);
        history.push_back(Z);
        history.push_back(S);
        history.push_back(Z);
        history.push_back(S);
        Randomizer {
            history: history
        }
    }

    pub fn next_piece(&mut self) -> Piecedef {
        let mut choices = vec!(S, Z, O, T, L, I, J);
        let mut rng = thread_rng();
        choices.shuffle(&mut rng);
        for _ in 0..3 {
            if self.history.contains(&choices[6]) {
                choices.shuffle(&mut rng)
            }
        }
        let piece = choices.pop().unwrap();
        let _ = self.history.pop_back();
        self.history.push_front(piece.clone());
        piece
    }
}
