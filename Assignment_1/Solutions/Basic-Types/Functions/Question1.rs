// Solution:

fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32  {
    x + y
}

/* 
Explantion/What I understood:
Added i32 to x and removed semi colon in x+y
*/