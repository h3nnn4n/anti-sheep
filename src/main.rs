mod cube;
mod defs;

fn main() {
    let mut c = cube::Cube::init();

    //let cubestring = "FFBLBRDLDUBRRFDDLRLUUUFB".to_string();
    let cubestring = "WOWOBBYORWBBRRYYGGRWYOGG".to_string(); // R' U

    c.from_string(cubestring);

    let solve_sequence = c.solve_double_headed_bfs();

    c.do_move_sequence(solve_sequence);

    assert!(c.is_solved());
}
