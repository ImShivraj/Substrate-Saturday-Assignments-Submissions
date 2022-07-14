// Solution:


struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    p.name = String::from("sunfei");
    println!("Success!");
}



/* 
Explantion/What I understood:

Added p.name to add string sunfei and it fixes the errors.

*/