use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    let mut floor = 0;
    let mut position = 0;
    let mut count = 0;
    
    match file.read_to_string(&mut s) {
        Err(_) => panic!(),
        Ok(_) => &s
    };

    let char_vec: Vec<char> = s.trim().chars().collect();

    for c in char_vec {
        count += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character"),
        }

        if floor == -1 && position == 0 {
            position = count;
            println!("Position: {}", position);
        }
    };

    println!("Final floor is: {floor}");
}