use std::io;


fn main() {
    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();
    let sequence = sequence.trim().to_string();
    println!("{}", sequence);
}
