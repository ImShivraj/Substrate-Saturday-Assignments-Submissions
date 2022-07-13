// Solution:

fn main() {
    never_return();
}

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    panic!("nothing!")
}
 

/* 
Explantion/What I understood:
Removing unccessary println and adding panic fixes it.

*/