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

fn translate(sequence: &Vec<String>) -> String {
    let mut peptide = String::new();
    for codon in sequence {
        peptide.push_str(match codon.as_str() {
            "gcg" | "gcc" | "gca" | "gct" => "A",
            "cgu" | "cgc" | "cga" | "cgg" | "aga" | "agg" => "R",
            "aau" | "aac" => "N",
            "gau" | "gac" => "D",
            "ugu" | "ugc" => "C",
            "caa" | "cag" => "Q",
            "gaa" | "gag" => "E",
            "ggu" | "ggc" | "gga" | "ggg" => "G",
            "cau" | "cac" => "H",
            "auu" | "auc" | "aua" => "I",
            "cuu" | "cuc" | "cua" | "cug" | "uua" | "uug" => "L",
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

fn verbose_translate(peptide: &String) -> String {
    let mut verbose_peptide = String::new();
    for amino_acid in peptide.chars() {
        verbose_peptide.push_str(match amino_acid {
            'A' => "Ala",
            'R' => "Arg",
            'N' => "Asn",
            'D' => "Asp",
            'C' => "Cys",
            'Q' => "Gln",
            'E' => "Glu",
            'G' => "Gly",
            'H' => "His",
            'I' => "Ile",
            'L' => "Leu",
            'K' => "Lys",
            'M' => "Met",
            'F' => "Phe",
            'P' => "Pro",
            'S' => "Ser",
            'T' => "Thr",
            'W' => "Trp",
            'Y' => "Tyr",
            'V' => "Val",
            '-' => "Stop",
            _ => "Unknown",
        });
        verbose_peptide.push_str(" ");
    }
    verbose_peptide
}
