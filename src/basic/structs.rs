#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub level: u8,
    pub attach: u8,
}

impl Monster {
    pub fn new(name: String, level: u8, attach: u8) -> Self {
        Monster {
            name,
            level,
            attach,
        }
    }

    pub fn attach(&self) {
        println!(
            "attached ==> name: {}, level: {}, attach: {}",
            self.name, self.level, self.attach
        )
    }

    pub fn reduce_attach(&mut self, level: u8) {
        self.attach -= level
    }
    pub fn level_up(&mut self, level: u8) {
        self.level += level
    }
}
