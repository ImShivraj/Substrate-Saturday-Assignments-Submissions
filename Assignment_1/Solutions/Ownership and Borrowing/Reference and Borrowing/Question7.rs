// Solution:


fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}




/* 
Explantion/What I understood:

added &s in r1 and r2 ana it works out.


*/