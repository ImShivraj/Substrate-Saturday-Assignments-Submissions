// Solution:


fn main() {

    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}





/* 
Explantion/What I understood:

used &mut in s to borrow object and it fixes the error.


*/