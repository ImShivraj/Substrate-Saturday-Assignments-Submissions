// Solution:


fn main() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");
    println!("Success!");
}


/* 
Explantion/What I understood:

Replaced index value ..3 in slice and it fixes the errors.


*/