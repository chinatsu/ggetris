use crate::point::Point;

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
