use std::io::{self, Read};
fn main() -> () {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let vec: Vec<&str> = buffer.split_whitespace().collect();
    let num: u32 = vec[0].parse::<u32>().unwrap() * vec[1].parse::<u32>().unwrap();
    let result = match num % 2 == 1{
        true => "Odd",
        false => "Even"
    };
    print!("{}", result)    
}