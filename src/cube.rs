extern crate rand;
use self::rand::Rng;
use defs;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    o: [i64; 8],
    p: [defs::Corner; 8],
}

impl Cube {
    pub fn print(&self) {
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

    pub fn solve(&mut self) {
        let mut q = vec![self.to_i()];

        loop {
            if self.is_solved() {
                return;
            }
        }
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
        for i in 0..8 {
            self.o[i] = self.o[i] % 3;
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
        let mut c2 = super::Cube::init();

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
        let mut c2 = super::Cube::init();

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
}
