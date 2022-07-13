// Question 9 Solution:


fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}


/* 
Explantion/What I understood:

We need to fill [3, 2] in the blank to run it successfully.

*/