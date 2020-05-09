use std::io::{self, Read};
fn main() -> () {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let rows: Vec<&str> = buffer.split("\n").collect();
    let a = rows[0];
    let bc: Vec<&str> = rows[1].split(" ").collect();
    let b = bc[0];
    let c = bc[1];
    let s = rows[2];
    let num = a.parse::<u32>().unwrap() + b.parse::<u32>().unwrap() + c.parse::<u32>().unwrap();
    print!("{} {}", num, s)
}
