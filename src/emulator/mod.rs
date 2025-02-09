mod cartridge;
mod cpu;
mod flags;
mod mmu;
mod ppu;

use cartridge::Cartridge;
use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;

use anyhow::Result;

pub struct Emulator {
    cpu: Cpu,
    cartridge: Cartridge,
    mmu: Mmu,
    ppu: ppu::Ppu,
    //     input: input::Joypad,
    //     timer: timer::Timer,
    //     interrupts: interrupts::InterruptController,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
            cartridge: Cartridge::new(),
            mmu: Mmu::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        self.cartridge.load(path)?;
        self.mmu.load_cartridge(&self.cartridge);
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.mmu, &mut self.ppu);
        }
    }
}
