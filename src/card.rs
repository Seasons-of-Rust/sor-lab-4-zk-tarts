use std::fmt;

use crate::FightResult;

/// A Card is a card stores a price, health, and damage.
pub struct Card {
    pub price: u32,
    pub health: u32,
    pub damage: u32,
}

impl Card {
    pub fn fight(&self, other: &Card) -> FightResult {
        use std::cmp::Ordering::*;
        match (
            self.damage.cmp(&other.health), // compare this cards damage to other cards hp
            other.damage.cmp(&self.health), // compare other cards damge to this cards hp
        ) {
            (Less, Less) => FightResult::Draw, // both damages are less than health
            (Greater | Equal, Less) => FightResult::Win, // only this cards damage is enough to win
            (Less, Greater | Equal) => FightResult::Loss, // only other cards damage is enough to win
            _ => FightResult::Tie,                        // cards both have enough damage to win
        }
    }

    /// Give a play by play of the battle
    pub fn print_fight(&self, other: &Card) -> FightResult {
        println!("\n{} vs {}", &self, other);
        println!("🗡️ 🗡️ 🗡️");

        let fight_result = self.fight(other);

        match fight_result {
            FightResult::Win => println!("{} wins!", self),
            FightResult::Loss => println!("{} wins!", other),
            FightResult::Tie => println!("It's a tie!"),
            FightResult::Draw => println!("It's a draw!"),
        }

        println!();

        fight_result
    }
}

/// Implement the Display trait for Card so that it can be printed. It will
/// print in the form:
///
/// |Card: dmg/hp|
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|Card: {}/{}|", self.damage, self.health)
    }
}
