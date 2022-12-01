use std::io;

fn main() {
    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();
    let sequence = sequence.trim().to_string().to_lowercase();
    let rna_sequence = convert_to_rna(&sequence);
    println!("{}", rna_sequence);
    let splitted = split_sequence(&rna_sequence);
    let peptide = translate(&splitted);
    println!("{:?}", splitted);
    println!("{}", peptide);

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

fn translate(sequence: &Vec<String>) -> String {
    let mut peptide = String::new();
    for codon in sequence {
        peptide.push_str(match codon.as_str() {
            "gcg" | "gcc" | "gca" | "gct" => "A",
            "cgu" | "cgc" | "cga" | "cgg" | "aga" | "agg" => "R",
            "aau"| "aac" => "N",
            "gau" | "gac" => "D",
            "ugu" | "ugc" => "C",
            "caa" | "cag" => "Q",
            "gaa" | "gag" => "E",
            "ggu" | "ggc" | "gga" | "ggg" => "G",
            "cau" | "cac" => "H",
            "auu" | "auc" | "aua" => "I",
            "cuu" | "cuc" | "cua" | "cug"|"uua"|"uug" => "L",
            "aaa" | "aag" => "K",
            "aug" => "M",
            "uuu" | "uuc" => "F",
            "ccu" | "ccc" | "cca" | "ccg" => "P",
            "ucu" | "ucc" | "uca" | "ucg" | "agu" | "agc" => "S",
            "acu" | "acc" | "aca" | "acg" => "T",
            "ugg" => "W",
            "uau" | "uac" => "Y",
            "guu" | "guc" | "gua" | "gug" => "V",
            "uaa" | "uag" | "uga" => "-",
            _ => "X",
        });
    }
    peptide
}