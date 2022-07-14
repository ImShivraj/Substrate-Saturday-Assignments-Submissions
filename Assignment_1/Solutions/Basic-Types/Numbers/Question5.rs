// Question 5 Solution:

fn main() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
 }

/* 
Explantion/What I understood:
changed 251_u8 to 247_u8 in v1 and add(251,8) to (119,8) in v2, since it is the max value of u8 and i8.

*/