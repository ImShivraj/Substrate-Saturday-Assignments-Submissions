// Solution:


fn main() {
    let mut s = String::from("hello");
     s.push(',');
     s.push_str(" world");
     s += "!";
 
     println!("{}", s)
 }



/* 
Explantion/What I understood:

push_str is used and it fixes the errors.

*/