// Solution:

fn main() {
    let s = String::from("hello, ");
    
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}


/* 
Explantion/What I understood:

Adding mut to s1 solves the problem as it make s1 mutable.


*/