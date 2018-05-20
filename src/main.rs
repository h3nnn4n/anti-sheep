mod cube;
mod defs;

fn main() {
    let mut c = cube::Cube::init();

    //c.random_shuffle(3);
    c.r_move();
    c.u_move();
    c.rp_move();
    c.up_move();

    let solve_sequence = c.solve();

    for m in solve_sequence.iter() {
        c.do_move(*m);
    }

    println!("{:?}", solve_sequence);

    c.print();
}
