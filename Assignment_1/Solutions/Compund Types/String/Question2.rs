// Solution:


fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
 }
 
 fn greetings(s: &str) {
     println!("{}",s)
 }




/* 
Explantion/What I understood:

changed the s to &s and it fixes the  error.

*/