// pub struct Allergies {
//     list: Vec<Allergen>,
// }
pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies: Vec<Allergen> = Vec::new();
        let bit_string = format!("{:08b}", score & 0xFF);
        let bits = bit_string.chars();

        for (ix, e) in bits
            .rev()
            .enumerate()
        {
            if e == '1' {
                allergies.push(match ix {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => unreachable!(),
                });
            }
        }
        Self(allergies)
        // Self { list: allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        for e in self
            .0
            .iter()
        {
            if e == allergen {
                return true;
            }
        }
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0
            .clone()
    }
}
