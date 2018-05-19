mod defs;
mod cube;

fn main() {
    let mut c = cube::Cube::init();

    c.print();
    c.r_move();
    c.r2_move();
    c.r_move();
    c.print();
}
