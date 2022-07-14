// Solution:


#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}", f.data);
} 




/* 
Explantion/What I understood:

Modified the println! to get f.data and it fixes the error.

*/