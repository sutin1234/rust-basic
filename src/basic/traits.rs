pub trait Player {
    fn attach(&self);
}

pub struct Mage {
    pub name: String,
    pub level: u8,
}
pub struct Warrior {
    pub name: String,
    pub level: u8,
}

impl Player for Mage {
    fn attach(&self) {
        println!("{} casting firebal", self.name)
    }
}

impl Player for Warrior {
    fn attach(&self) {
        println!("{} swinging a sword", self.name)
    }
}

pub fn excute_attach<T: Player>(player: &T) {
    player.attach();
}
