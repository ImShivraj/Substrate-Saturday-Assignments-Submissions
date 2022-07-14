// Solution:


fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}




/* 
Explantion/What I understood:

used s1.clone() and &s2 to get s3 and it fixes the error.

*/