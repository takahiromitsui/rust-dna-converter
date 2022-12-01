pub mod helpers {
    pub fn convert_to_rna(sequence: &String) -> String {
        sequence.replace("t", "u")
    }
    pub fn split_sequence(sequence: &String) -> Vec<String> {
        sequence
            .as_bytes()
            .chunks(3)
            .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
            .collect()
    }
    pub fn translate(sequence: &Vec<String>) -> String {
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
}

#[cfg(test)]
mod tests {
    use crate::helpers::*;

    #[test]
    fn t_will_be_u() {
        let dna_sequence = String::from("atgtggaagagcaaagtaggatttt");
        let rna_sequence = convert_to_rna(&dna_sequence);
        assert_eq!(rna_sequence, "auguggaagagcaaaguaggauuuu");
    }

    #[test]
    fn length_should_be_one_third() {
        let random_sequence = String::from_utf8(vec![b'a'; 100]).unwrap();
        let splitted = split_sequence(&random_sequence);
        assert_eq!(splitted.len(), 34);
    }

    #[test]
    fn should_translate() {
        let sequence = vec![
            String::from("aug"),
            String::from("ugg"),
            String::from("aag"),
            String::from("aug"),
            String::from("ugg"),
            String::from("aag"),
            String::from("aug"),
            String::from("ugg"),
            String::from("aag"),
        ];
        let peptide = translate(&sequence);
        assert_ne!(peptide, "MMM");
        assert_eq!(peptide, "MWKMWKMWK");
    }
}
