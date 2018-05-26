use std::slice::Iter;

//#[derive(Copy, Clone, Debug)]
//pub enum Facelet {
//U1, // = 0
//U2, // = 1
//U3, // = 2
//U4, // = 3
//R1, // = 4
//R2, // = 5
//R3, // = 6
//R4, // = 7
//F1, // = 8
//F2, // = 9
//F3, // = 10
//F4, // = 11
//D1, // = 12
//D2, // = 13
//D3, // = 14
//D4, // = 15
//L1, // = 16
//L2, // = 17
//L3, // = 18
//L4, // = 19
//B1, // = 20
//B2, // = 21
//B3, // = 22
//B4, // = 23
//}

//#[derive(Copy, Clone, Debug)]
//pub enum Color {
//U, // = 0
//R, // = 1
//F, // = 2
//D, // = 3
//L, // = 4
//B, // = 5
//}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

impl Move {
    pub fn iterator_ftm() -> Iter<'static, Move> {
        static MOVES: [Move; 6] = [Move::U1, Move::U3, Move::R1, Move::R3, Move::F1, Move::F3];

        MOVES.into_iter()
    }

    pub fn iterator_htm() -> Iter<'static, Move> {
        static MOVES: [Move; 9] = [
            Move::U1,
            Move::U2,
            Move::U3,
            Move::R1,
            Move::R2,
            Move::R3,
            Move::F1,
            Move::F2,
            Move::F3,
        ];

        MOVES.into_iter()
    }

    pub fn print_formated(seq: Vec<Move>) {
        let formated = Move::format_move_sequence(seq);

        for m in formated.iter() {
            print!("{} ", *m);
        }

        println!();
    }

    pub fn reverse_move_sequence(seq: Vec<Move>) -> Vec<Move> {
        let mut reverse: Vec<Move> = Vec::new();

        for mov in seq.iter() {
            reverse.push(match *mov {
                Move::U1 => Move::U3,
                Move::U2 => Move::U2,
                Move::U3 => Move::U1,
                Move::R1 => Move::R3,
                Move::R2 => Move::R2,
                Move::R3 => Move::R1,
                Move::F1 => Move::F3,
                Move::F2 => Move::F2,
                Move::F3 => Move::F1,
            })
        }

        reverse.reverse();

        reverse
    }

    pub fn format_move_sequence(seq: Vec<Move>) -> Vec<String> {
        let simplified_move_sequece = Move::simplify_move_sequence(seq);
        let mut formated: Vec<String> = Vec::new();

        for mov in simplified_move_sequece.iter() {
            formated.push(match *mov {
                Move::U1 => "U ".to_string(),
                Move::U2 => "U2".to_string(),
                Move::U3 => "U'".to_string(),
                Move::R1 => "R ".to_string(),
                Move::R2 => "R2".to_string(),
                Move::R3 => "R ".to_string(),
                Move::F1 => "F'".to_string(),
                Move::F2 => "F2".to_string(),
                Move::F3 => "F'".to_string(),
            })
        }

        formated
    }

    pub fn simplify_move_sequence(seq: Vec<Move>) -> Vec<Move> {
        let mut seq2: Vec<Move> = Vec::new();

        for (k, _) in seq.iter().enumerate() {
            if k < seq.len() - 1 {
                if seq[k] != seq[k + 1] {
                    if k == 0 || seq[k - 1] != seq[k] {
                        seq2.push(seq[k]);
                    }
                } else {
                    seq2.push(match seq[k] {
                        Move::R1 => Move::R2,
                        Move::U1 => Move::U2,
                        Move::F1 => Move::F2,
                        _ => panic!("Unexpected repeated move"),
                    })
                }
            } else {
                if seq[k] != seq[k - 1] {
                    seq2.push(seq[k]);
                }
            }
        }

        seq2
    }
}

//pub const cornerFacelet: [[Facelet; 3]; 8] = [
//[Facelet::U4, Facelet::R1, Facelet::F2],
//[Facelet::U3, Facelet::F1, Facelet::L2],
//[Facelet::U1, Facelet::L1, Facelet::B2],
//[Facelet::U2, Facelet::B1, Facelet::R2],
//[Facelet::D4, Facelet::R4, Facelet::B3],
//[Facelet::D2, Facelet::F4, Facelet::R3],
//[Facelet::D1, Facelet::L4, Facelet::F3],
//[Facelet::D3, Facelet::B4, Facelet::L3],
//];

//pub const cornerColor: [[Color; 3]; 8] = [
//[Color::U, Color::R, Color::F],
//[Color::U, Color::F, Color::L],
//[Color::U, Color::L, Color::B],
//[Color::U, Color::B, Color::R],
//[Color::D, Color::R, Color::B],
//[Color::D, Color::F, Color::R],
//[Color::D, Color::L, Color::F],
//[Color::D, Color::B, Color::L],
//];

//pub const N_MOVE: i64 = 9; //  number of possible face moves
//pub const N_TWIST: i64 = 729; //  3^6 possible corner orientations
//pub const N_CORNERS: i64 = 5040; //  7! corner permutations in phase 2

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
        _ => panic!("Invalid Corner ID"),
    }
}

pub fn int_to_move(n: i32) -> Move {
    match n {
        0 => Move::U1,
        1 => Move::U2,
        2 => Move::U3,
        3 => Move::R1,
        4 => Move::R2,
        5 => Move::R3,
        6 => Move::F1,
        7 => Move::F2,
        8 => Move::F3,
        _ => panic!("Invalid Move ID"),
    }
}

#[cfg(test)]
mod tests {
    use cube;

    #[test]
    fn reverse() {
        let mut c1 = cube::Cube::init();
        let mut c2 = cube::Cube::init();

        for _ in 0..5 {
            c1.random_shuffle(8);

            let solve_sequence = c1.solve();
            let mut reverse_solve_sequence =
                super::Move::reverse_move_sequence(solve_sequence.clone());

            c1.copy(c2);

            c1.do_move_sequence(solve_sequence);
            c1.do_move_sequence(reverse_solve_sequence);

            assert_eq!(c1, c2);
        }
    }
}
