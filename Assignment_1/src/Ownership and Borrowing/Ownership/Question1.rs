// Solution:

fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

// OR

fn main() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
 

/* 
Explantion/What I understood:
clone() method solved this !
*/