//! Piece definitions for each of the pieces

use ggez::graphics::Rect;
use super::point::Point;

lazy_static! {
    pub static ref PIECES: [Piecedef; 7] = [S, Z, O, T, L, I, J];
}

#[derive(Clone, Copy, Debug)]
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
            Point { x: -1, y: -1 },
            Point { x: -1, y: 0 }
        ],
        [
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 0 }
        ],
        [
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 }
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

pub fn get_offset(id: char) -> Rect {
    match id {
        'z' => get_cell(2),
        'l' => get_cell(2),
        'o' => get_cell(3),
        's' => get_cell(2),
        'i' => get_cell(4),
        'j' => get_cell(4),
        't' => get_cell(4),
        'g' => get_cell(7),
        _ =>   get_cell(9)
    }
}

fn get_cell(val: u32) -> Rect {
    Rect::new(0.0, val as f32 * 32.0 / 384.0, 1.0, 32.0/384.0)
}
