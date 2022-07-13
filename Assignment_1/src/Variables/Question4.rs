// Question 4 Solution:

fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

/* 
Explantion/What I understood:

We have to give a .to_string() function to the variable that we want to use in the other funtion.

*/