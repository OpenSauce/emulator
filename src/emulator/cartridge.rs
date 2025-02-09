use anyhow::Result;
use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge { rom: Vec::new() }
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read_to_end(&mut self.rom)?;
        Ok(())
    }
}
