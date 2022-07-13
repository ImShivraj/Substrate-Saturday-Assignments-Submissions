// Solution:


struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "coding".to_string()
    };
    println!("Success!");
} 



/* 
Explantion/What I understood:

Added hobby to the main fn and it fixes the  error.

*/