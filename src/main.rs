mod cube;
mod defs;

use std::time::Instant;

fn main() {
    let mut c = cube::Cube::init();
    let c2 = cube::Cube::init();

    c.random_shuffle(20);

    let start_t = Instant::now();
    let solve_sequence = c.solve();
    let end_t = Instant::now();

    let delta = end_t - start_t;
    println!(
        "solver took: {}.{}s",
        delta.as_secs(),
        delta.subsec_millis()
    );

    for m in solve_sequence.iter() {
        c.do_move(*m);
    }

    println!("{:?}", solve_sequence);

    assert_eq!(c, c2);
}
