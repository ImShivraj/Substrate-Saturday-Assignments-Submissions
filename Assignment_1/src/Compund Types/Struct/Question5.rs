// Solution:


struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}


/* 
Explantion/What I understood:

Added name in fn build_person and it fixes the errors.


*/