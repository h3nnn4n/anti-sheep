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

    pub fn u2_move(&mut self) {
        self.u_move();
        self.u_move();
    }

    pub fn u_move(&mut self) {
        self.p = [
            self.p[3], self.p[0], self.p[1], self.p[2], self.p[4], self.p[5], self.p[6], self.p[7],
        ];
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_move_eq() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.r_move();
        assert_ne!(c, c2);
        c.r2_move();
        assert_ne!(c, c2);
        c.r_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn u_move_eq() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.u_move();
        assert_ne!(c, c2);
        c.u2_move();
        assert_ne!(c, c2);
        c.u_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn f_move_eq() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.f_move();
        assert_ne!(c, c2);
        c.f2_move();
        assert_ne!(c, c2);
        c.f_move();

        assert_eq!(c, c2);
    }
}
