use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub name: String,
    pub strength: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl Default for Card {
    fn default() -> Self {
        Card { 
            name: "".to_owned(),
            strength: 1,
            health: 1,
            cost: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        }
    }
}

impl Card {
    pub fn builder(name: String) -> CardBuilder {
        CardBuilder::new(name)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct CardBuilder {
    pub name: String,
    pub strength: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new(name: String) -> Self {
        CardBuilder {
            name,            
            ..Default::default(); // this is so awesome!
        }
    }
    pub fn strength(mut self, s: i32) -> Self {
        self.strength = Some(s);
        self
    }
    pub fn trigger(mut self, t: Trigger, s: String) -> Self {
        self.triggers.insert(t, s);
        self
    }
    pub fn build(self) -> Card {
        Card {
            name: self.name,
            strength: self.strength.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}

#[cfg(test)]
mod test_builder {
    use super::*;
    #[test]
    pub fn test_card_builder() {
        let given = Card::builder("General Blight".to_string())
            .strength(4)
            .trigger(Trigger::BattleCry, "Deal 2 Damage")
            .build();
        let mut triggers = BTreeMap::new();
        triggers.insert(Trigger::BattleCry, "Deal 2 Damage".to_string());
        let expected = Card {
            name: "General Blight".to_string(),
            strength: 4,
            cost: 1,
            health: 1,
            triggers,
            abilities: Vec::new(),
        };
        assert_eq!(expected, given)
    }
}
