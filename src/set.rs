use crate::champion::Champion;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Set {
    pub champions: Vec<Champion>,
}

impl Set {
    pub fn get_champions(&self) -> Vec<Champion> {
        let mut champions = Vec::<Champion>::new();

        for champion in self.champions.iter() {
            if champion.traits.is_empty() {
                continue;
            }

            champions.push(champion.clone())
        }

        champions
    }
}

#[cfg(test)]
mod tests {
    mod get_champions {
        use crate::champion::Champion;
        use crate::set::Set;
        use spectral::prelude::*;

        #[test]
        fn returns_the_champions() {
            let set = Set {
                champions: vec![Champion {
                    name: "Olaf".to_string(),
                    cost: 1,
                    traits: vec!["Axtwerfer".to_string()],
                }],
            };

            let champions = set.get_champions();
            assert_that!(&champions).has_length(1);
            assert_that!(&champions.get(0).unwrap().name).is_equal_to("Olaf".to_string());
        }

        #[test]
        fn ignores_champions_without_traits() {
            let set = Set {
                champions: vec![Champion {
                    name: "Olaf".to_string(),
                    cost: 1,
                    traits: vec![],
                }],
            };

            let champions = set.get_champions();
            assert_that!(&champions).has_length(0);
        }
    }
}
