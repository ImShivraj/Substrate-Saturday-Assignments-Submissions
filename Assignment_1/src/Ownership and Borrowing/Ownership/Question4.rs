// Solution:

fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


/* 
Explantion/What I understood:
clone() method solved this !

*/