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

    fn fix_orientation(&mut self) {
        for i in 0..8 {
            self.o[i] = self.o[i] % 3;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r_move() {
        let mut c = super::Cube::init();
        let mut c2 = super::Cube::init();

        c.r_move();
        c.r2_move();
        c.r_move();

        assert_eq!(c, c2);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
