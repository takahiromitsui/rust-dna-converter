pub mod helpers {
    pub fn convert_to_rna(sequence: &String) -> String {
        sequence.replace("t", "u")
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
}
