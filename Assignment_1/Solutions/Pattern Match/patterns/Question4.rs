// Solution:

fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
    println!("Success!");
}


/* 
Explantion/What I understood:
Added if condition in the blank.

*/