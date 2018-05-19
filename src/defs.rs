#[derive(Copy, Clone, Debug)]
pub enum Facelet {
    U1, // = 0
    U2, // = 1
    U3, // = 2
    U4, // = 3
    R1, // = 4
    R2, // = 5
    R3, // = 6
    R4, // = 7
    F1, // = 8
    F2, // = 9
    F3, // = 10
    F4, // = 11
    D1, // = 12
    D2, // = 13
    D3, // = 14
    D4, // = 15
    L1, // = 16
    L2, // = 17
    L3, // = 18
    L4, // = 19
    B1, // = 20
    B2, // = 21
    B3, // = 22
    B4, // = 23
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    U, // = 0
    R, // = 1
    F, // = 2
    D, // = 3
    L, // = 4
    B, // = 5
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Corner {
    URF = 0,
    UFL = 1,
    ULB = 2,
    UBR = 3,
    DRB = 4,
    DFR = 5,
    DLF = 6,
    DBL = 7,
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    U1, // = 0
    U2, // = 1
    U3, // = 2
    R1, // = 3
    R2, // = 4
    R3, // = 5
    F1, // = 6
    F2, // = 7
    F3, // = 8
}

pub const cornerFacelet: [[Facelet; 3]; 8] = [
    [Facelet::U4, Facelet::R1, Facelet::F2],
    [Facelet::U3, Facelet::F1, Facelet::L2],
    [Facelet::U1, Facelet::L1, Facelet::B2],
    [Facelet::U2, Facelet::B1, Facelet::R2],
    [Facelet::D4, Facelet::R4, Facelet::B3],
    [Facelet::D2, Facelet::F4, Facelet::R3],
    [Facelet::D1, Facelet::L4, Facelet::F3],
    [Facelet::D3, Facelet::B4, Facelet::L3],
];

pub const cornerColor: [[Color; 3]; 8] = [
    [Color::U, Color::R, Color::F],
    [Color::U, Color::F, Color::L],
    [Color::U, Color::L, Color::B],
    [Color::U, Color::B, Color::R],
    [Color::D, Color::R, Color::B],
    [Color::D, Color::F, Color::R],
    [Color::D, Color::L, Color::F],
    [Color::D, Color::B, Color::L],
];

pub const N_MOVE: i64 = 9; //  number of possible face moves
pub const N_TWIST: i64 = 729; //  3^6 possible corner orientations
pub const N_CORNERS: i64 = 5040; //  7! corner permutations in phase 2

pub fn int_to_corner(n: i32) -> Corner {
    match n {
        0 => Corner::URF,
        1 => Corner::UFL,
        2 => Corner::ULB,
        3 => Corner::UBR,
        4 => Corner::DRB,
        5 => Corner::DFR,
        6 => Corner::DLF,
        7 => Corner::DBL,
        _ => panic!("Invalid corner ID"),
    }
}
