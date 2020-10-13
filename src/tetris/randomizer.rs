use rand::{thread_rng, seq::SliceRandom};
use super::piecedefs::{PIECES, Piecedef, S, Z};
use std::collections::VecDeque;

pub struct Randomizer {
    history: VecDeque<char>
}

impl Randomizer {
    pub fn new() -> Randomizer {
        Randomizer {
            history: VecDeque::from([S, Z, S, Z].iter().map(|x| x.id).collect::<Vec<char>>())
        }
    }

    pub fn next_piece(&mut self) -> Option<Piecedef> {
        let mut rng = thread_rng();
        let mut choice = PIECES.choose(&mut rng)?;
        for _ in 0..3 {
            if self.history.contains(&choice.id) {
                choice = PIECES.choose(&mut rng)?;
            }
        }
        let _ = self.history.pop_front();
        self.history.push_back(choice.id);
        Some(*choice)
    }
}
