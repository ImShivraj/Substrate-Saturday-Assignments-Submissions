// Question 5 Solution:

fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

/* 
Explantion/What I understood:

The first x gets shadowed by the second one. So we can declare a new variable with the same name as a previous variable.

*/