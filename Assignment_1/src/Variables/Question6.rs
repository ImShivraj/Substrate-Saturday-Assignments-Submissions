// Question 6 Solution:

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    // x += 3;   [removed]


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

/* 
Explantion/What I understood:

Removing the unwanted fields fixes the problem.

*/