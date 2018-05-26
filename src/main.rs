mod cube;
mod defs;

use std::time::Instant;

fn main() {
    let mut c = cube::Cube::init();

    //for _ in 0..20 {
    loop {
        c.random_shuffle(50);

        let t1 = Instant::now();
        let solve_sequence = c.solve_forward_bfs();
        let e1 = Instant::now();

        let t2 = Instant::now();
        let solve_sequence2 = c.solve_reverse_bfs();
        let e2 = Instant::now();

        let t3 = Instant::now();
        let solve_sequence3 = c.solve_double_headed_bfs();
        let e3 = Instant::now();

        let s1 = e1 - t1;
        let s2 = e2 - t2;
        let s3 = e3 - t3;

        println!(
            "{}.{} {}.{} {}.{}",
            s1.as_secs(),
            s1.subsec_millis(),
            s2.as_secs(),
            s2.subsec_millis(),
            s3.as_secs(),
            s3.subsec_millis(),
        );
    }
}
