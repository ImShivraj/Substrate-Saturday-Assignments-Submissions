// Question 9 Solution:

fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}



/* 
Explantion/What I understood:

c had an unassigned datatype so added u8 and sum==-5.

*/