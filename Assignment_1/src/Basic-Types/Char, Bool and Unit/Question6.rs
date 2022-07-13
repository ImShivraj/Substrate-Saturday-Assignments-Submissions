// Solution:

use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

/* 
Explantion/What I understood:
The value if changed to 0, there's no error.
*/