// Solution:

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world"); 
    s
}


/* 
Explantion/What I understood:
Removing the inappropriate lines fixes the error.

*/