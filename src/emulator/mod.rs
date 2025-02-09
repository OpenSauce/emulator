pub mod cpu;
pub mod flags;

use anyhow::Result;

pub struct Emulator {
    cpu: cpu::Cpu,
    //     mmu: mmu::MMU,
    //     ppu: ppu::PPU,
    //     input: input::Joypad,
    //     timer: timer::Timer,
    //     interrupts: interrupts::InterruptController,
    //     cartridge: cartridge::Cartridge,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: cpu::Cpu::new(),
            //         mmu: mmu::MMU::new(),
            //         ppu: ppu::PPU::new(),
            //         input: input::Joypad::new(),
            //         timer: timer::Timer::new(),
            //         interrupts: interrupts::InterruptController::new(),
            //         cartridge: cartridge::Cartridge::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        print!("Loading ROM: {}", path);
        Ok(())
    }

    pub fn run(&mut self) {
        print!("Running emulator...");
    }
}
