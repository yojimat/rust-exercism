pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    UnknowAllergen,
}

impl Allergies {
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = vec![];
        for item in &self.allergens {
            if self.is_allergic_to(item) {
                allergies.push(match_allergen(item));
            }
        }
        allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    fn allergies_from_score(score: u32) -> Vec<Allergen> {
        let mut allergies = vec![];
        let mut score = score;

        if score % 2 == 1 {
            allergies.push(Allergen::Eggs);
            score -= 1
        }

        while score > 0 {
            let (allergie, allergie_number) = select_by_key_number(score);

            if !allergies.contains(&allergie) {
                allergies.push(allergie);
            } 

            score -= allergie_number;
        }
        allergies
    }

    pub fn new(score: u32) -> Self {
        let allergens = Self::allergies_from_score(score);
        Allergies { allergens }
    }
}

fn select_by_key_number(score: u32) -> (Allergen, u32) {
    use Allergen::*;
    match score {
        score if score >= 128 => (Cats, 128),
        64..=127 => (Pollen, 64),
        32..=63 => (Chocolate, 32),
        16..=31 => (Tomatoes, 16),
        8..=15 => (Strawberries, 8),
        4..=7 => (Shellfish, 4),
        2 | 3 => (Peanuts, 2),
        _ => (Eggs, 1),
    }
}

fn match_allergen(item: &Allergen) -> Allergen {
    use Allergen::*;
    match item {
        Eggs => Eggs,
        Peanuts => Peanuts,
        Shellfish => Shellfish,
        Strawberries => Strawberries,
        Tomatoes => Tomatoes,
        Chocolate => Chocolate,
        Pollen => Pollen,
        Cats => Cats,
        UnknowAllergen => UnknowAllergen,
    }
}
