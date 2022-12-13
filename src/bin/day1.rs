use std::io;
use std::io::Read;

fn main() {

    let mut directions = String::new();
    let mut end: i32 = 0;

    println!("Input directions");

    io::stdin().read_to_string(&mut directions)
        .expect("Failed to read");

    for ch in directions.chars(){
        if ch == '('{
            end += 1;
        }else if ch == ')'{
            end -= 1;
        }
    }
    println!("Ended on floor {}", end);
}
