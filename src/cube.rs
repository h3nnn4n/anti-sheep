extern crate rand;
use self::rand::Rng;
use defs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    o: [i64; 8],
    p: [defs::Corner; 8],
}

impl Cube {
    pub fn _print(&self) {
        println!("{:?} {:?}", self.o, self.p);
    }

    pub fn init() -> Cube {
        Cube {
            o: [0; 8],
            p: [
                defs::Corner::URF, // 0
                defs::Corner::UFL, // 1
                defs::Corner::ULB, // 2
                defs::Corner::UBR, // 3
                defs::Corner::DRB, // 4
                defs::Corner::DFR, // 5
                defs::Corner::DLF, // 6
                defs::Corner::DBL, // 7
            ],
        }
    }

    pub fn reset(&mut self) {
        *self = Cube::init();
    }

    /* This nice description was copied from https://github.com/hkociemba/Rubiks2x2x2-OptimalSolver/blob/master/face.py
     *
     *
     *    The names of the facelet positions of the cube
     *              |********|
     *              |*U1**U2*|
     *              |********|
     *              |*U3**U4*|
     *              |********|
     *     |********|********|********|********|
     *     |*L1**L2*|*F1**F2*|*R1**R2*|*B1**B2*|
     *     |********|********|********|********|
     *     |*L3**L4*|*F3**F4*|*R3**R4*|*B3**B4*|
     *     |********|********|********|********|
     *              |********|
     *              |*D1**D2*|
     *              |********|
     *              |*D3**D4*|
     *              |********|
     *
     *    U1, U2, U3, U4, R1, R2, R3, R4, F1, F2, F3, F4,
     *    D1, D2, D3, D4, L1, L2, L3, L4, B1, B2, B3, B4
     *
     *    A cube definition string "UBL..." means for example: In position U1 we have the U-color, in position U2 we have the
     *    B-color, in position U3 we have the L color etc. according to the order U1, U2, U3, U4, R1, R2, R3, R4, F1, F2, F3,
     *    F4, D1, D2, D3, D4, L1, L2, L3, L4, B1, B2, B3, B4 of the enum constants.
     *
     */

    pub fn from_string(&mut self, input: String) {
        assert_eq!(input.chars().count(), 24);

        for m in ['W', 'R', 'B', 'Y', 'O', 'G'].iter() {
            assert_eq!(input.matches(*m).count(), 4);
        }

        let mut o = [-1; 8];
        let mut p = vec![
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
            defs::Corner::URF,
        ];

        for i in 0..8 {
            let corner = defs::int_to_corner(i);
            let cf = defs::cornerFacelet[i as usize];
            let w = (0..3)
                .map(|x| input.chars().nth(cf[x] as usize).unwrap())
                .collect::<String>();

            println!("{}", w);
        }

        return;
    }

    pub fn from_i(&mut self, (_p, _o): (i64, i64)) {
        let mut p = _p;
        let mut o = _o;

        for c in 0..8 {
            self.p[c] = defs::int_to_corner((p % 8) as i32);
            p /= 8;
        }

        for c in 0..8 {
            self.o[c] = o % 3;
            o /= 3;
        }
    }

    pub fn to_i(&self) -> (i64, i64) {
        let mut p = 0;
        let mut o = 0;

        for (k, v) in self.p.iter().enumerate() {
            p += (*v as i64) * 8_i64.pow(k as u32);
        }

        for (k, v) in self.o.iter().enumerate() {
            o += (*v as i64) * 3_i64.pow(k as u32);
        }

        (p, o)
    }

    pub fn solve(&self) -> Vec<defs::Move> {
        //self.solve_forward_bfs()
        //self.solve_reverse_bfs()
        self.solve_double_headed_bfs()
    }

    pub fn solve_double_headed_bfs(&self) -> Vec<defs::Move> {
        let mut forward_queue: VecDeque<((i64, i64), defs::Move)> = VecDeque::new();
        let mut reverse_queue: VecDeque<((i64, i64), defs::Move)> = VecDeque::new();
        let mut reverse_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut forward_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut forward_visited: HashSet<(i64, i64)> = HashSet::new();
        let mut reverse_visited: HashSet<(i64, i64)> = HashSet::new();
        let mut path: Vec<defs::Move> = vec![];
        let mut c = Cube::init();

        forward_queue.push_back((self.to_i(), defs::Move::I0));
        reverse_queue.push_back((Cube::init().to_i(), defs::Move::I0));

        let iter = if true {
            defs::Move::iterator_ftm()
        } else {
            defs::Move::iterator_htm()
        };

        'main_loop: loop {
            // Forward BFS
            if forward_queue.len() == 0 {
                panic!("Cube is unsolvable");
            } else {
                let (c_i, last_move) = forward_queue.pop_front().unwrap();

                c.from_i(c_i);

                for m in iter.clone() {
                    if defs::redundant_move(*m, last_move) {
                        continue;
                    }

                    let mut c2 = c.get_copy();
                    c2.do_move(*m);

                    if forward_visited.contains(&c2.to_i()) && reverse_visited.contains(&c2.to_i())
                    {
                        forward_path.insert(c2.to_i(), (c.to_i(), *m));
                        c.copy(c2);
                        break 'main_loop;
                    } else {
                        if !forward_visited.contains(&c2.to_i()) {
                            forward_queue.push_back((c2.to_i(), *m));
                            forward_path.insert(c2.to_i(), (c.to_i(), *m));
                            forward_visited.insert(c2.to_i());
                        }
                    }
                }
            }

            //backward BFS
            if reverse_queue.len() == 0 {
                panic!("Cube is unsolvable");
            } else {
                let (c_i, last_move) = reverse_queue.pop_front().unwrap();

                c.from_i(c_i);

                for m in iter.clone() {
                    if defs::redundant_move(*m, last_move) {
                        continue;
                    }

                    let mut c2 = c.get_copy();
                    c2.do_move(*m);

                    if forward_visited.contains(&c2.to_i()) && reverse_visited.contains(&c2.to_i())
                    {
                        reverse_path.insert(c2.to_i(), (c.to_i(), *m));
                        c.copy(c2);
                        break 'main_loop;
                    } else {
                        if !reverse_visited.contains(&c2.to_i()) {
                            reverse_queue.push_back((c2.to_i(), *m));
                            reverse_path.insert(c2.to_i(), (c.to_i(), *m));
                            reverse_visited.insert(c2.to_i());
                        }
                    }
                }
            }
        }

        let mut path_forward: Vec<defs::Move> = vec![];
        let mut m: defs::Move;

        let mut k = c.to_i();
        let target = self.to_i();

        while k != target {
            let a = *forward_path.get(&k).unwrap();

            k = a.0;
            m = a.1;
            path_forward.push(m);
        }

        path_forward.reverse();

        let mut path_backward: Vec<defs::Move> = vec![];

        let target = Cube::init().to_i();

        let mut k = c.to_i();

        while k != target {
            let a = *reverse_path.get(&k).unwrap();
            k = a.0;
            m = a.1;
            path_backward.push(m);
        }

        path_backward.reverse();
        path_backward = defs::Move::reverse_move_sequence(path_backward);

        path.extend(&path_forward);
        path.extend(&path_backward);

        path
    }

    pub fn solve_reverse_bfs(&self) -> Vec<defs::Move> {
        let mut reverse_queue: VecDeque<((i64, i64), defs::Move)> = VecDeque::new();
        let mut reverse_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut reverse_visited: HashSet<(i64, i64)> = HashSet::new();
        let mut c = Cube::init();

        reverse_queue.push_back((Cube::init().to_i(), defs::Move::I0));

        let iter = if true {
            defs::Move::iterator_ftm()
        } else {
            defs::Move::iterator_htm()
        };

        loop {
            if reverse_queue.len() == 0 {
                panic!("Cube is unsolvable");
            } else {
                let (c_i, last_move) = reverse_queue.pop_front().unwrap();

                c.from_i(c_i);

                for m in iter.clone() {
                    if defs::redundant_move(*m, last_move) {
                        continue;
                    }

                    let mut c2 = c.get_copy();
                    c2.do_move(*m);

                    if c2 == *self {
                        reverse_path.insert(c2.to_i(), (c.to_i(), *m));

                        let mut path: Vec<defs::Move> = vec![];

                        let mut m: defs::Move;

                        let mut k = self.to_i();
                        let target = Cube::init().to_i();

                        while k != target {
                            let a = *reverse_path.get(&k).unwrap();
                            k = a.0;
                            m = a.1;
                            path.push(m);
                        }

                        path.reverse();

                        return defs::Move::reverse_move_sequence(path);
                    } else {
                        if !reverse_visited.contains(&c2.to_i()) {
                            reverse_queue.push_back((c2.to_i(), *m));
                            reverse_path.insert(c2.to_i(), (c.to_i(), *m));
                            reverse_visited.insert(c2.to_i());
                        }
                    }
                }
            }
        }
    }

    pub fn solve_forward_bfs(&self) -> Vec<defs::Move> {
        let mut q: VecDeque<((i64, i64), defs::Move)> = VecDeque::new();
        let mut reverse_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        let mut c = Cube::init();

        q.push_back((self.to_i(), defs::Move::I0));

        let iter = if true {
            defs::Move::iterator_ftm()
        } else {
            defs::Move::iterator_htm()
        };

        loop {
            if q.len() == 0 {
                panic!("Cube is unsolvable");
            } else {
                let (c_i, last_move) = q.pop_front().unwrap();

                c.from_i(c_i);

                for m in iter.clone() {
                    if defs::redundant_move(*m, last_move) {
                        continue;
                    }

                    let mut c2 = c.get_copy();
                    c2.do_move(*m);

                    if c2.is_solved() {
                        reverse_path.insert(c2.to_i(), (c_i, *m));

                        let mut path: Vec<defs::Move> = vec![];

                        let target = self.to_i();
                        let mut m: defs::Move;
                        let mut k = Cube::init().to_i();

                        while k != target {
                            let a = *reverse_path.get(&k).unwrap();
                            k = a.0;
                            m = a.1;
                            path.push(m);
                        }

                        path.reverse();

                        return path;
                    } else {
                        if !visited.contains(&c2.to_i()) {
                            q.push_back((c2.to_i(), *m));
                            reverse_path.insert(c2.to_i(), (c_i, *m));
                            visited.insert(c2.to_i());
                        }
                    }
                }
            }
        }
    }

    pub fn copy(&mut self, c: Cube) {
        self.from_i(c.to_i());
    }

    pub fn get_copy(&self) -> Cube {
        let mut c = Cube::init();
        c.copy(*self);
        c
    }

    pub fn rp_move(&mut self) {
        self.o = [
            self.o[3] + 2,
            self.o[1],
            self.o[2],
            self.o[4] + 1,
            self.o[5] + 2,
            self.o[0] + 1,
            self.o[6],
            self.o[7],
        ];

        self.p = [
            self.p[3], self.p[1], self.p[2], self.p[4], self.p[5], self.p[0], self.p[6], self.p[7],
        ];

        self.fix_orientation();
    }

    pub fn r2_move(&mut self) {
        self.o = [
            self.o[4], self.o[1], self.o[2], self.o[5], self.o[0], self.o[3], self.o[6], self.o[7],
        ];

        self.p = [
            self.p[4], self.p[1], self.p[2], self.p[5], self.p[0], self.p[3], self.p[6], self.p[7],
        ];

        self.fix_orientation();
    }

    pub fn r_move(&mut self) {
        self.o = [
            self.o[5] + 2,
            self.o[1],
            self.o[2],
            self.o[0] + 1,
            self.o[3] + 2,
            self.o[4] + 1,
            self.o[6],
            self.o[7],
        ];

        self.p = [
            self.p[5], self.p[1], self.p[2], self.p[0], self.p[3], self.p[4], self.p[6], self.p[7],
        ];

        self.fix_orientation();
    }

    pub fn up_move(&mut self) {
        self.o = [
            self.o[1], self.o[2], self.o[3], self.o[0], self.o[4], self.o[5], self.o[6], self.o[7],
        ];

        self.p = [
            self.p[1], self.p[2], self.p[3], self.p[0], self.p[4], self.p[5], self.p[6], self.p[7],
        ];
    }

    pub fn u2_move(&mut self) {
        self.o = [
            self.o[2], self.o[3], self.o[0], self.o[1], self.o[4], self.o[5], self.o[6], self.o[7],
        ];

        self.p = [
            self.p[2], self.p[3], self.p[0], self.p[1], self.p[4], self.p[5], self.p[6], self.p[7],
        ];
    }

    pub fn u_move(&mut self) {
        self.o = [
            self.o[3], self.o[0], self.o[1], self.o[2], self.o[4], self.o[5], self.o[6], self.o[7],
        ];

        self.p = [
            self.p[3], self.p[0], self.p[1], self.p[2], self.p[4], self.p[5], self.p[6], self.p[7],
        ];
    }

    pub fn fp_move(&mut self) {
        self.o = [
            self.o[5] + 1,
            self.o[0] + 2,
            self.o[2],
            self.o[3],
            self.o[4],
            self.o[6] + 2,
            self.o[1] + 1,
            self.o[7],
        ];

        self.p = [
            self.p[5], self.p[0], self.p[2], self.p[3], self.p[4], self.p[6], self.p[1], self.p[7],
        ];

        self.fix_orientation();
    }

    pub fn f2_move(&mut self) {
        self.o = [
            self.o[6], self.o[5], self.o[2], self.o[3], self.o[4], self.o[1], self.o[0], self.o[7],
        ];

        self.p = [
            self.p[6], self.p[5], self.p[2], self.p[3], self.p[4], self.p[1], self.p[0], self.p[7],
        ];
    }

    pub fn f_move(&mut self) {
        self.o = [
            self.o[1] + 1,
            self.o[6] + 2,
            self.o[2],
            self.o[3],
            self.o[4],
            self.o[0] + 2,
            self.o[5] + 1,
            self.o[7],
        ];

        self.p = [
            self.p[1], self.p[6], self.p[2], self.p[3], self.p[4], self.p[0], self.p[5], self.p[7],
        ];

        self.fix_orientation();
    }

    fn fix_orientation(&mut self) {
        let s: i64 = self.o.iter().sum();
        assert!(s % 3 == 0);

        for i in 0..8 {
            assert!(self.o[i] >= 0);

            self.o[i] = self.o[i] % 3;
        }
    }

    pub fn do_move(&mut self, m: defs::Move) {
        match m {
            defs::Move::R1 => self.r_move(),
            defs::Move::R2 => self.r2_move(),
            defs::Move::R3 => self.rp_move(),

            defs::Move::U1 => self.u_move(),
            defs::Move::U2 => self.u2_move(),
            defs::Move::U3 => self.up_move(),

            defs::Move::F1 => self.f_move(),
            defs::Move::F2 => self.f2_move(),
            defs::Move::F3 => self.fp_move(),

            _ => (),
        }
    }

    pub fn random_move(&mut self) {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0, 9);

        self.do_move(defs::int_to_move(n));
    }

    pub fn random_shuffle(&mut self, n: i32) {
        let mut rng = rand::thread_rng();
        let mut m = rng.gen_range(0, 9);
        let mut last = defs::int_to_move(m);
        let mut mov = last;

        for _ in 0..n {
            last = mov;
            while mov == last {
                m = rng.gen_range(0, 9);
                mov = defs::int_to_move(m);
            }

            self.do_move(mov);
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Cube::init()
    }

    pub fn do_move_sequence(&mut self, seq: Vec<defs::Move>) {
        for m in seq.iter() {
            self.do_move(*m);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn r_move() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.r_move();
        assert_ne!(c, c2);
        c.r2_move();
        assert_ne!(c, c2);
        c.r_move();

        assert_eq!(c, c2);

        c.rp_move();
        c.rp_move();
        c.rp_move();
        c2.r_move();

        assert_eq!(c, c2);

        c.reset();
        c2.reset();

        c.r2_move();
        c.r2_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn u_move() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.u_move();
        assert_ne!(c, c2);
        c.u2_move();
        assert_ne!(c, c2);
        c.u_move();

        assert_eq!(c, c2);

        c.up_move();
        c.up_move();
        c.up_move();
        c2.u_move();

        assert_eq!(c, c2);

        c.reset();
        c2.reset();

        c.u2_move();
        c.u2_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn f_move() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.f_move();
        assert_ne!(c, c2);
        c.f2_move();
        assert_ne!(c, c2);
        c.f_move();

        assert_eq!(c, c2);

        c2.f_move();
        c2.f_move();
        c2.f_move();
        c.fp_move();

        assert_eq!(c, c2);

        c.reset();
        c2.reset();

        c.f2_move();
        c.f2_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn seq_test() {
        let mut c = super::Cube::init();
        let c2 = super::Cube::init();

        for _ in 0..6 {
            // R U R' U'
            c.r_move();
            c.u_move();
            c.rp_move();
            c.up_move();
        }

        assert_eq!(c, c2);
    }

    #[test]
    fn random_shuffle() {
        let mut c = super::Cube::init();
        let c2 = super::Cube::init();

        for _ in 0..50 {
            c.reset();
            c.random_shuffle(50);

            assert_ne!(c, c2);
        }
    }

    #[test]
    fn is_solved() {
        let mut c = super::Cube::init();

        assert!(c.is_solved());

        c.random_shuffle(50);

        assert!(!c.is_solved());
    }

    #[test]
    fn encode_decode() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        for _ in 0..100 {
            c.random_shuffle(50);
            c2.from_i(c.to_i());
            assert_eq!(c, c2);
        }
    }

    #[test]
    #[should_panic]
    fn from_string_wrong() {
        let mut c = super::Cube::init();
        c.from_string("HAHA".to_string());
    }

    #[test]
    fn compare_solver_forward_reverse() {
        let mut c1 = super::Cube::init();
        let mut c2 = super::Cube::init();

        for _ in 0..10 {
            c1.random_shuffle(5);
            c2.copy(c1);

            let solve_sequence1 = c1.solve_reverse_bfs();
            let solve_sequence2 = c2.solve_forward_bfs();

            c1.do_move_sequence(solve_sequence1);
            c2.do_move_sequence(solve_sequence2);

            assert_eq!(c1, c2);
        }
    }

    #[test]
    fn compare_solver_forward_double_headed() {
        let mut c1 = super::Cube::init();
        let mut c2 = super::Cube::init();

        for _ in 0..10 {
            c1.random_shuffle(5);
            c2.copy(c1);

            let solve_sequence1 = c1.solve_double_headed_bfs();
            let solve_sequence2 = c2.solve_forward_bfs();

            c1.do_move_sequence(solve_sequence1);
            c2.do_move_sequence(solve_sequence2);

            assert_eq!(c1, c2);
        }
    }

    #[test]
    fn compare_solver_reverse_double_headed() {
        let mut c1 = super::Cube::init();
        let mut c2 = super::Cube::init();

        for _ in 0..10 {
            c1.random_shuffle(5);
            c2.copy(c1);

            let solve_sequence1 = c1.solve_double_headed_bfs();
            let solve_sequence2 = c2.solve_reverse_bfs();

            c1.do_move_sequence(solve_sequence1);
            c2.do_move_sequence(solve_sequence2);

            assert_eq!(c1, c2);
        }
    }

    fn solve(n: i32) -> bool {
        let mut c = super::Cube::init();
        let c2 = super::Cube::init();

        c.random_shuffle(n);

        let solve_sequence = c.solve();

        c.do_move_sequence(solve_sequence);

        c == c2
    }

    macro_rules! solve_tests {
    ($($name:ident: $value:expr,)*) => { $(
        #[test]
        fn $name() {
            let n = $value;
            for _ in 0..5 {
                assert!(solve(n));
            }
        })*}
    }

    solve_tests! {
        solve_1: 1,
        solve_2: 2,
        solve_3: 3,
        solve_4: 4,
        solve_5: 5,
        solve_6: 6,
        solve_7: 7,
        solve_8: 8,
        solve_9: 9,
        solve_10: 10,
    }
}
