#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };
        (self.score & allergen_score) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_allergens = [
            (Allergen::Eggs, 1),
            (Allergen::Peanuts, 2),
            (Allergen::Shellfish, 4),
            (Allergen::Strawberries, 8),
            (Allergen::Tomatoes, 16),
            (Allergen::Chocolate, 32),
            (Allergen::Pollen, 64),
            (Allergen::Cats, 128),
        ];
        all_allergens
            .into_iter()
            .filter(|&(_, score)| (self.score & score) != 0)
            .map(|(allergen, _)| allergen)
            .collect()
    }
}
