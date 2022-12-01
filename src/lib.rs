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

    pub fn verbose_translate(peptide: &String) -> String {
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

    #[test]
    fn should_verbose_translate() {
        let peptide = String::from("MWKMWKMWK-*-RAS");
        let verbose_peptide = verbose_translate(&peptide);
        assert_eq!(
            verbose_peptide,
            "Met Trp Lys Met Trp Lys Met Trp Lys Stop Unknown Stop Arg Ala Ser "
        );
    }
}
