// Solution:

enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

 

/* 
Explantion/What I understood:
using matching method solved this in the blank
*/