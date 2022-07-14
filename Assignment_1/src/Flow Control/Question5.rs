// Solution:


fn main() {
    let a = [4, 3, 2, 1];

    // iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}



/* 
Explantion/What I understood:

.iter().enumerate() is used to get the data iterated and it solves the issue.


*/