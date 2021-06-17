#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceLocation {
    pub piece: Piece,
    pub rotation: Rotation,
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rotation {
    North,
    West,
    South,
    East,
}

impl Piece {
    pub const fn cells(self) -> [(i8, i8); 4] {
        match self {
            Piece::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            Piece::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Piece::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            Piece::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            Piece::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            Piece::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Piece::Z => [(-1, 1), (0, 0), (1, 0), (0, 1)],
        }
    }

    pub const fn sensible_rotations(self) -> &'static [Rotation] {
        match self {
            Piece::O => &[Rotation::North],
            Piece::I | Piece::S | Piece::Z => &[Rotation::North, Rotation::East],
            Piece::T | Piece::L | Piece::J => &[
                Rotation::North,
                Rotation::East,
                Rotation::South,
                Rotation::West,
            ],
        }
    }
}

impl Rotation {
    pub const fn rotate_cell(self, (x, y): (i8, i8)) -> (i8, i8) {
        match self {
            Rotation::North => (x, y),
            Rotation::East => (y, -x),
            Rotation::South => (-x, -y),
            Rotation::West => (-y, x),
        }
    }
}

impl PieceLocation {
    pub const fn cells(self) -> [(i8, i8); 4] {
        let cells = self.piece.cells();
        [
            self.translate(self.rotation.rotate_cell(cells[0])),
            self.translate(self.rotation.rotate_cell(cells[1])),
            self.translate(self.rotation.rotate_cell(cells[2])),
            self.translate(self.rotation.rotate_cell(cells[3])),
        ]
    }

    const fn translate(self, (x, y): (i8, i8)) -> (i8, i8) {
        (x + self.x, y + self.y)
    }
}
