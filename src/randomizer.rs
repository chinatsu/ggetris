/// A rudimentary bag-randomizer for Tetris

extern crate rand;
use rand::{thread_rng, Rng};
use piecedefs::*;

pub struct Randomizer {
    bag: Vec<Piecedef>
}

impl Randomizer {
    pub fn new() -> Randomizer {
        let bag = Randomizer::new_bag();
        Randomizer {
            bag: bag
        }
    }

    /// Takes a new piece out of the bag
    pub fn next_piece(&mut self) -> Piecedef {
        if self.bag.len() == 0 {
            self.bag = Randomizer::new_bag();
        }
        self.bag.pop().unwrap()
    }


    /// Creates a new permutated bag
    fn new_bag() -> Vec<Piecedef> {
        let mut bag = vec!(T, L, I, J, S, Z, O);
        let mut rng = thread_rng();
        let num: usize = rng.gen_range(0, 4);
        let mut tail = vec!(bag[num]);
        for i in 0..bag.len() as usize {
            if bag[i].id == tail[0].id {
                bag.remove(i);
                break;
            }
        }
        rng.shuffle(&mut bag.as_mut_slice());
        bag.append(&mut tail);
        for x in bag.iter().rev() {
            print!("{} ", x.id);
        }
        println!();
        bag
    }
}
