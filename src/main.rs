use std::time::Instant;

mod cube;
mod defs;

fn main() {
    let mut c = cube::Cube::init();

    //let cubestring = "FFBLBRDLDUBRRFDDLRLUUUFB".to_string();
    //let cubestring = "WOWOBBYORWBBRRYYGGRWYOGG".to_string(); // R' U

    //c.from_string(cubestring);

    c.random_shuffle(30);

    let t_start = Instant::now();
    let solve_sequence = c.solve();
    let t_end = Instant::now();

    let diff = t_end.duration_since(t_start);
    println!("{:?}.{:?}", diff.as_secs(), diff.subsec_nanos());

    c.do_move_sequence(solve_sequence);

    assert!(c.is_solved());
}
