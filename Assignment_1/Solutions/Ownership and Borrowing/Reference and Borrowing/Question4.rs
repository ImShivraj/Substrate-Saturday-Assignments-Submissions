// Solution:


fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
 


/* 
Explantion/What I understood:

added &mut to get s and it solves the error.


*/