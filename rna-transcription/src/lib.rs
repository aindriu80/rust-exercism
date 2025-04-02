#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'T' | 'G' | 'C' => continue,
                _ => return Err(i),
            }
        }
        Ok(Dna {
            sequence: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_seq = self
            .sequence
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!("Dna::new ensures only valid nucleotides"),
            })
            .collect::<String>();

        Rna { sequence: rna_seq }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'U' | 'G' | 'C' => continue,
                _ => return Err(i),
            }
        }
        Ok(Rna {
            sequence: rna.to_string(),
        })
    }
}
