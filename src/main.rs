mod cube;
mod defs;

use std::time::Instant;

fn main() {
    let mut c = cube::Cube::init();
    let c2 = cube::Cube::init();

    //c.random_shuffle(20);
    c.r_move();
    c.u_move();
    //c.rp_move();
    //c.up_move();

    let start_t = Instant::now();
    println!("Solution: ");
    let solve_sequence = c.solve_forward_bfs();
    println!("{:?}", solve_sequence.clone());
    let solve_sequence2 = c.solve_reverse_bfs();
    println!("{:?}", solve_sequence2.clone());
    let solve_sequence3 = c.solve_double_headed_bfs();
    println!("{:?}", solve_sequence3.clone());
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

    //defs::Move::print_formated(solve_sequence);

    assert_eq!(c, c2);
}
