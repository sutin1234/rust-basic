#[derive(Debug)]
pub enum PlayerEnum {
    WARRIOR(i32),
    MAGE(i32),
}

impl PlayerEnum {
    pub fn get_weapon(&self) {
        match self {
            PlayerEnum::WARRIOR(d) => println!("weapon is: {} damage: {}", "Warrior", d),
            PlayerEnum::MAGE(d) => println!("weapon is: {} damage: {}", "Warrior", d),
        }
    }
}
