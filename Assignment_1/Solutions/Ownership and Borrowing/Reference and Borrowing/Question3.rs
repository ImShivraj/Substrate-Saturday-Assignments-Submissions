// Solution:


fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

 


/* 
Explantion/What I understood:

&s is replaced and it solves the error.


*/