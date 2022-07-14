// Solution:


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    println!("Success!");
} 



/* 
Explantion/What I understood:

Added {x: 1, y: 1} in msg to get its data in the blank and it fixes the error.

*/