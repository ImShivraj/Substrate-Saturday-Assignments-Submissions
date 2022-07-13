// Solution:


fn main() {
    let mut s = String::from("hello, ");

    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}

 


/* 
Explantion/What I understood:

added &mut to get s and it solves the issue.


*/