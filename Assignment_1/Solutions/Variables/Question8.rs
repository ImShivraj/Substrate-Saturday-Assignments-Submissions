// Question 8 Solution:


fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}


/* 
Explantion/What I understood:

using 'mut' makes the variable mutable and we can change the value of the variable later.

*/