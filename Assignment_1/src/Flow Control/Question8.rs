// Solution:


fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);
    println!("Success!");
}



/* 
Explantion/What I understood:

continue is used and break is used to come out of {} is used to fill the blank.


*/