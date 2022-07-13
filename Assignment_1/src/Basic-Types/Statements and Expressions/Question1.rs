// Solution:

fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
    println!("Success!");
 }
 

/* 
Explantion/What I understood:
x needs to be added to the main function to avoid the error.

*/