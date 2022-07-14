// Question 3 Solution:

fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x);
}


/* 
Explantion/What I understood:

Since y was declared inside the scope, so it cannot be ised out of the scope. This is why the error was showing. 

*/