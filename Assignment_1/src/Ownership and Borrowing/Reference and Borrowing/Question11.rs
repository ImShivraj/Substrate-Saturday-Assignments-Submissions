// Solution:


fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    // let r2 = &mut s;

    r1.push_str("world");
}


/* 
Explantion/What I understood:

Added r1.push_str() and it fixes the compiler error.

*/