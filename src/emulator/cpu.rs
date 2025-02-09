use core::panic;

use crate::emulator::flags::FlagsRegister;
use crate::emulator::mmu::Mmu;
use crate::emulator::ppu::Ppu;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFFE,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, _ppu: &mut Ppu) {
        let op_code = mmu.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let instruction = match Instruction::from_byte(op_code) {
            Some(instruction) => instruction,
            None => return,
        };

        self.execute_instruction(mmu, instruction);
    }

    fn execute_instruction(&mut self, _mmu: &mut Mmu, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmaticTarget::A => {
                    let new_value = self.add(self.registers.a);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::B => {
                    let new_value = self.add(self.registers.b);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::C => {
                    let new_value = self.add(self.registers.c);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::D => {
                    let new_value = self.add(self.registers.d);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::E => {
                    let new_value = self.add(self.registers.e);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::H => {
                    let new_value = self.add(self.registers.h);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::L => {
                    let new_value = self.add(self.registers.l);
                    self.registers.a = new_value;
                }
                ArithmaticTarget::HL => {
                    panic!("ADD HL Not implemented");
                }
                _ => panic!("Unknown target for ADD instruction"),
            },
            Instruction::NOP() => (),
            _ => (),
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}

enum Instruction {
    ADD(ArithmaticTarget),
    NOP(),
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::NOP()),
            0x80 => Some(Instruction::ADD(ArithmaticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmaticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmaticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmaticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmaticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmaticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmaticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmaticTarget::A)),
            _ => None,
        }
    }
}

enum ArithmaticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::default(),
            h: 0,
            l: 0,
        }
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
