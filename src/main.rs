//use rand::{random, thread_rng, Rng};
#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // murs : [nord, sud, est, ouest]
}
fn main() ->() {
    println!("test du premier algo de Maze appeller \"Prim\"");
}
fn GenerateMaze(width:u32, height:u32) ->() {
    //let grille = thread_rng().gen(i32);
}
