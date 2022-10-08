#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(index, character)| match character {
                'A' | 'C' | 'G' | 'T'=> Ok(character),
                _ => Err(index),
            }).collect::<Result<String, usize>>().map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(index, character)| match character {
                'A' | 'C' | 'G' | 'U'=> Ok(character),
                _ => Err(index),
            }).collect::<Result<String, usize>>().map(Rna)
    }
}
