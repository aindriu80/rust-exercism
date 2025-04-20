pub fn translate(rna: &str) -> Option<Vec<&str>> {
    if rna.is_empty() {
        return Some(vec![]);
    }

    let mut proteins = Vec::new();
    let mut found_stop = false;

    // Process RNA in groups of 3 characters (codons)
    for i in (0..rna.len()).step_by(3) {
        // Check if we have a complete codon
        if i + 3 > rna.len() {
            // If this is an incomplete codon at the end, return None
            // unless we already found a stop codon
            return if found_stop { Some(proteins) } else { None };
        }

        let codon = &rna[i..i + 3];

        match of_codon(codon) {
            Some("STOP") => {
                found_stop = true;
                break; // Stop translation but continue to check if there are incomplete codons
            }
            Some(protein) => proteins.push(protein),
            None => return None, // Invalid codon
        }
    }

    Some(proteins)
}

fn of_codon(codon: &str) -> Option<&str> {
    match codon {
        "AUG" => Some("Methionine"),
        "UUU" | "UUC" => Some("Phenylalanine"),
        "UUA" | "UUG" => Some("Leucine"),
        "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
        "UAU" | "UAC" => Some("Tyrosine"),
        "UGU" | "UGC" => Some("Cysteine"),
        "UGG" => Some("Tryptophan"),
        "UAA" | "UAG" | "UGA" => Some("STOP"),
        _ => None,
    }
}
