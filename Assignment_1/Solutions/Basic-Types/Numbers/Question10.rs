// Question 10 Solution:

use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("Success!");
}



/* 
Explantion/What I understood:

Filled in the blanks with 5 and =5 respectively. =5 bcs it get included by using =

*/