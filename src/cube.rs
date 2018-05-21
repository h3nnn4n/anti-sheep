extern crate rand;
use self::rand::Rng;
use defs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

    pub fn from_string(&mut self, input: String) {
        assert!(input.chars().count() == 24);
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
        self.solve_bfs()
        //self.solve_double_headed_bfs()
    }

    pub fn solve_double_headed_bfs(&self) -> Vec<defs::Move> {
        let mut forward_queue: VecDeque<(i64, i64)> = VecDeque::new();
        let mut reverse_queue: VecDeque<(i64, i64)> = VecDeque::new();
        let mut reverse_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut forward_visited: HashSet<(i64, i64)> = HashSet::new();
        let mut reverse_visited: HashSet<(i64, i64)> = HashSet::new();
        let mut c = Cube::init();

        forward_queue.push_back(self.to_i());
        reverse_queue.push_back(Cube::init().to_i());

        let iter = if true {
            defs::Move::iterator_htm()
        } else {
            defs::Move::iterator_ftm()
        };

        loop {
            //// Forward BFS
            //if forward_queue.len() == 0 {
            //panic!("Cube is unsolvable");
            //} else {
            //c.from_i(forward_queue.pop_front().unwrap());

            //for m in iter.clone() {
            //let mut c2 = c.get_copy();
            //c2.do_move(*m);

            //if c2.is_solved() {
            //let mut path: Vec<defs::Move> = vec![];

            //return path;
            //} else {
            //if !forward_visited.contains(&c2.to_i()) {
            //forward_queue.push_back(c2.to_i());
            //reverse_path.insert(c2.to_i(), (c.to_i(), *m));
            //forward_visited.insert(c2.to_i());
            //}
            //}
            //}
            //}

            //backward BFS
            if reverse_queue.len() == 0 {
                panic!("Cube is unsolvable");
            } else {
                //println!("Hhaa");
                c.from_i(reverse_queue.pop_front().unwrap());
                //println!("{:?}", c);

                for m in iter.clone() {
                    let mut c2 = c.get_copy();
                    c2.do_move(*m);

                    if c2 == *self {
                        reverse_path.insert(c2.to_i(), (c.to_i(), *m));

                        let mut path: Vec<defs::Move> = vec![];

                        let mut k = self.to_i();
                        let mut m: defs::Move;
                        let target = Cube::init().to_i();

                        while k != target {
                            let a = *reverse_path.get(&k).unwrap();
                            k = a.0;
                            m = a.1;
                            path.push(m);
                        }

                        //path.reverse();

                        return path;
                    } else {
                        if !reverse_visited.contains(&c2.to_i()) {
                            reverse_queue.push_back(c2.to_i());
                            reverse_path.insert(c2.to_i(), (c.to_i(), *m));
                            reverse_visited.insert(c2.to_i());
                        }
                    }
                }
            }
        }
    }

    pub fn solve_bfs(&self) -> Vec<defs::Move> {
        let mut q: VecDeque<(i64, i64)> = VecDeque::new();
        //let mut forward_path: HashMap<((i64, i64), defs::Move), (i64, i64)> = HashMap::new();
        let mut reverse_path: HashMap<(i64, i64), ((i64, i64), defs::Move)> = HashMap::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        let mut c = Cube::init();

        q.push_back(self.to_i());

        loop {
            c.from_i(q.pop_front().unwrap());

            let c_i = c.to_i();

            let iter = if true {
                defs::Move::iterator_htm()
            } else {
                defs::Move::iterator_ftm()
            };

            for m in iter {
                let mut c2 = c.get_copy();
                c2.do_move(*m);

                if c2.is_solved() {
                    //forward_path.insert((c_i, m), c2.to_i());
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
                        q.push_back(c2.to_i());
                        //forward_path.insert((c_i, *m), c2.to_i());
                        reverse_path.insert(c2.to_i(), (c_i, *m));
                        visited.insert(c2.to_i());
                    }
                }
            }

            if q.len() == 0 {
                panic!("Cube is unsolvable");
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
        self.r_move();
        self.r2_move();
    }

    pub fn r2_move(&mut self) {
        self.r_move();
        self.r_move();
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
        self.u_move();
        self.u2_move();
    }

    pub fn u2_move(&mut self) {
        self.u_move();
        self.u_move();
    }

    pub fn u_move(&mut self) {
        self.o = [
            self.o[3], self.o[0], self.o[1], self.o[2], self.o[4], self.o[5], self.o[6], self.o[7],
        ];

        self.p = [
            self.p[3], self.p[0], self.p[1], self.p[2], self.p[4], self.p[5], self.p[6], self.p[7],
        ];

        self.fix_orientation();
    }

    pub fn fp_move(&mut self) {
        self.f_move();
        self.f_move();
        self.f_move();
    }

    pub fn f2_move(&mut self) {
        self.f_move();
        self.f_move();
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
        }
    }

    pub fn random_move(&mut self) {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0, 9);

        match n {
            0 => self.r_move(),
            1 => self.r2_move(),
            2 => self.rp_move(),

            3 => self.u_move(),
            4 => self.u2_move(),
            5 => self.up_move(),

            6 => self.f_move(),
            7 => self.f2_move(),
            8 => self.fp_move(),

            _ => self.random_move(),
        }
    }

    pub fn random_shuffle(&mut self, n: i32) {
        for _ in 0..n {
            self.random_move();
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Cube::init()
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

        c.random_shuffle(5);

        assert_ne!(c, c2);
    }

    #[test]
    fn is_solved() {
        let mut c = super::Cube::init();

        assert!(c.is_solved());

        c.random_shuffle(10);

        assert!(!c.is_solved());
    }

    #[test]
    fn encode_decode() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        for _ in 0..100 {
            c.random_shuffle(11);
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

    fn solve(n: i32) -> bool {
        let mut c = super::Cube::init();
        let c2 = super::Cube::init();
        c.random_shuffle(n);

        let solve_sequence = c.solve();

        for m in solve_sequence.iter() {
            c.do_move(*m);
        }

        c == c2
    }

    macro_rules! solve_tests {
    ($($name:ident: $value:expr,)*) => { $(
        #[test]
        fn $name() {
            let n = $value;
            for _ in 0..3 {
                assert!(solve(n));
            }
        })*}
    }

    solve_tests! {
        solve_1: 1,
        solve_2: 2,
        solve_3: 3,
        solve_4: 4,
        //solve_5: 5,
        //solve_6: 6,
        //solve_7: 7,
    }
}
