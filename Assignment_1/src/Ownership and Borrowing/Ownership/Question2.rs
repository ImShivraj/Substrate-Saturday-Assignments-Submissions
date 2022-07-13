// Solution:

fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String)-> String {
    println!("{}", s);
    s
}


/* 
Explantion/What I understood:
Adding string method in function take_ownership fixes this issue.
*/