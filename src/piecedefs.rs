//! Piece definitions for each of the pieces

extern crate ggez;
use rand::distributions::Weighted;
use ggez::graphics::Color;
use point::*;

/// A Piecedef struct only contains its shapes and an ID
/// used to determine color. The ID is a char for the purpose
/// of clearing up which piece is which as they all are commonly
/// referred to as a single letter.
/// The shapes are defined as offset points from an origin, and
/// is stored in a list with its other rotation states
#[derive(Clone, Copy)]
pub struct Piecedef {
    pub shape: [[Point; 4]; 4],
    pub id: char
}

pub const T: Piecedef = Piecedef {
    shape: [
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: -1 }
        ],
        [
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 }
        ],
        [
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: -1, y: 0 }
        ]
    ],
    id: 't'
};

pub const L: Piecedef = Piecedef {
    shape: [
        [
            Point { x: 1, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 }
        ],
        [
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 }
        ],
    ],
    id: 'l'
};

pub const O: Piecedef = Piecedef {
    shape: [
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 }
        ]
    ],
    id: 'o'
};

pub const S: Piecedef = Piecedef {
    shape: [
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 }
        ],
        [
            Point { x: -1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: -1, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 }
        ]
    ],
    id: 's'
};

pub const Z: Piecedef = Piecedef {
    shape: [
        [
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 }
        ],
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: 1 }
        ]
    ],
    id: 'z'
};

pub const J: Piecedef = Piecedef {
    shape: [
        [
            Point { x: -1, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 }
        ],
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 }
        ],
        [
            Point { x: -1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 }
        ]
    ],
    id: 'j'
};

pub const I: Piecedef = Piecedef {
    shape: [
        [
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -1 }
        ],
        [
            Point { x: 1, y: -2 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
        ],
        [
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 }
        ],
        [
            Point { x: 0, y: -2 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
        ]
    ],
    id: 'i'
};

pub const PIECES: [Piecedef; 7] = [T, L, O, S, Z, J, I];

/// The randomizer's weights are defined here.
/// TODO: Replace randomizer with a bag-randomizer
pub const WEIGHTS: [Weighted<usize>; 7] = [
    Weighted { weight: 100, item: 0 },
    Weighted { weight: 100, item: 1 },
    Weighted { weight: 100, item: 2 },
    Weighted { weight: 100, item: 3 },
    Weighted { weight: 100, item: 4 },
    Weighted { weight: 100, item: 5 },
    Weighted { weight: 100, item: 6 },
];

/// A utility function to get a piece's color based on its ID.
pub fn get_color(id: char) -> Color {
    match id {
        't' => Color::new(0.8, 0.0, 1.0, 1.0),
        'l' => Color::new(1.0, 0.4, 0.0, 1.0),
        'o' => Color::new(1.0, 1.0, 0.0, 1.0),
        's' => Color::new(0.0, 0.8, 0.0, 1.0),
        'z' => Color::new(1.0, 0.0, 0.0, 1.0),
        'j' => Color::new(0.0, 0.0, 0.8, 1.0),
        'i' => Color::new(0.0, 1.0, 1.0, 1.0),
        _ => Color::new(0.0, 0.0, 0.0, 0.0)
    }
}
