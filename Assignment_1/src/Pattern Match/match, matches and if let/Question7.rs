// Solution:

enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}



/* 
Explantion/What I understood:

Added if let condition in the blank.



*/