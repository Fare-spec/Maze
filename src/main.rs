//use rand::{random, thread_rng, Rng};
#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // murs : [nord, sud, est, ouest]
}
impl Cell {
    fn new() -> Self{
        Cell {
            visited:false,
            walls:[true,true,true,true],
        }
    }
}
fn main() ->() {
    println!("nothing here");
}
fn generate_maze(width:u32, height:u32) ->() {
    //let grille = thread_rng().gen(i32);
}
