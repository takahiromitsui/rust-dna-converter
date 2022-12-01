use std::io::{self};

use na_converter::helpers::*;

fn main() {
    println!("Enter your DNA sequence:");
    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();
    let sequence = sequence.trim().to_string().to_lowercase();
    let rna_sequence = convert_to_rna(&sequence);
    let splitted = split_sequence(&rna_sequence);
    let peptide = translate(&splitted);
    let verbose_peptide = verbose_translate(&peptide);

    println!("\nRNA:\n{}", rna_sequence);
    println!("\nAmino Acid Sequence:\n{}", peptide);
    println!("\nVerbose Amino Acid Sequence:\n{}", verbose_peptide);
}
