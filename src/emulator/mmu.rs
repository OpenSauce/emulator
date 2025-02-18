use crate::emulator::cartridge::Cartridge;

pub struct Mmu {
    pub memory: [u8; 0x10000],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            memory: [0; 0x10000],
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &Cartridge) {
        self.memory[..cartridge.rom.len()].copy_from_slice(&cartridge.rom);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address as usize >= self.memory.len() {
            panic!("Invalid memory address: {:X}", address);
        }

        self.memory[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, value: u8) {
        if address as usize >= self.memory.len() {
            panic!("Invalid memory address: {:X}", address);
        }

        self.memory[address as usize] = value;
    }
}
