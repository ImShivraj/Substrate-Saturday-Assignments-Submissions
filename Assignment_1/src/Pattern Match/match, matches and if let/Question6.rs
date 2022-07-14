// Solution:


fn main() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}


/* 
Explantion/What I understood:

Changed the match statement to if let 



*/