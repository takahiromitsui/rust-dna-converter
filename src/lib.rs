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
}
