mod cube;
mod defs;

fn main() {
    let mut c = cube::Cube::init();

    c.random_shuffle(3);
    c.solve();
}
