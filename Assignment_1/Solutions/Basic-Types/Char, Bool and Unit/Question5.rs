// Solution:

fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());
    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

/* 
Explantion/What I understood:
changed all the _v and v to v0, it fixed the error.

*/