// Solution:


fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // The type of value is &mut String
       value => value.push_str(" world!") 
    }
}


/* 
Explantion/What I understood:
in match condition, the mut isnt used, we can remove the &mut inside match r

*/