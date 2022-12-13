use std::io;
use std::io::Read;

fn main() {

    let mut directions = String::new();
    let mut end = 0;

    let mut pos = 1;

    io::stdin().read_to_string(&mut directions)
        .expect("Failed to read");

    for ch in directions.chars(){
        if end == -1 {
            println!("Went to basement with previous step {}", pos-1);
            break;
        }
        if ch == '('{
            end += 1;
        }else if ch == ')'{
            end -= 1;
        }
        pos += 1;
    }
}
