use std::io;

fn main() {
    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();
    let sequence = sequence.trim().to_string().to_lowercase();
    let rna_sequence = convert_to_rna(&sequence);
    println!("{}", rna_sequence);
    let splitted = split_sequence(&rna_sequence);
    println!("{:?}", splitted);
}

fn convert_to_rna(sequence: &String) -> String {
    sequence.replace("t", "u")
}

fn split_sequence(sequence: &String) -> Vec<String> {
    sequence
        .as_bytes()
        .chunks(3)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}
