use core::fmt;
use std::collections::HashMap;
use std::thread;
use std::time;

use crate::emulator::flags::FlagsRegister;
use crate::emulator::mmu::Mmu;
use crate::emulator::ppu::Ppu;

const MEMORY_SIZE: u16 = 0xFFFF;
const PREFIXED_OPCODE: u8 = 0xCB;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
    success: i32,
    fail: i32,
    running: HashMap<Instruction, i32>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFFE,
            success: 0,
            fail: 0,
            running: HashMap::new(),
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, _ppu: &mut Ppu) {
        let mut instruction_byte = mmu.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        if self.pc == MEMORY_SIZE {
            println!("{:?}", self);
            thread::sleep(time::Duration::from_secs(5));
            return;
        }

        let prefixed = instruction_byte == PREFIXED_OPCODE;
        if prefixed {
            instruction_byte = mmu.read_byte(self.pc);
        }

        let instruction = match Instruction::from_byte(instruction_byte, prefixed) {
            Some(instruction) => instruction,
            None => {
                println!("Unknown instruction: {:X}", instruction_byte);
                self.fail += 1;
                return;
            }
        };

        self.success += 1;
        println!("Executing instruction: {:?}", instruction);

        self.running
            .entry(instruction.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.execute_instruction(mmu, instruction);
    }

    fn execute_instruction(&mut self, _mmu: &mut Mmu, instruction: Instruction) {
        match instruction {
            Instruction::Nop() => (),
            Instruction::Halt() => (),
            Instruction::Add(target) => match target {
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
                    println!("Add HL Not implemented");
                }
            },
            Instruction::Jp() => {
                println!("JPNZ Not implemented");
            }
            Instruction::Rlc(_target) => {
                println!("RLC Not implemented");
            }
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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Success: {}, fail: {}, running: {:?}",
            self.success, self.fail, self.running
        )
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Instruction {
    Add(ArithmaticTarget),
    Nop(),
    Jp(),
    Halt(),
    Rlc(ArithmaticTarget),
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Self> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_non_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::Rlc(ArithmaticTarget::B)),
            _ => None,
        }
    }

    fn from_byte_non_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::Nop()),
            0x76 => Some(Instruction::Halt()),
            0x80 => Some(Instruction::Add(ArithmaticTarget::B)),
            0x81 => Some(Instruction::Add(ArithmaticTarget::C)),
            0x82 => Some(Instruction::Add(ArithmaticTarget::D)),
            0x83 => Some(Instruction::Add(ArithmaticTarget::E)),
            0x84 => Some(Instruction::Add(ArithmaticTarget::H)),
            0x85 => Some(Instruction::Add(ArithmaticTarget::L)),
            0x86 => Some(Instruction::Add(ArithmaticTarget::HL)),
            0x87 => Some(Instruction::Add(ArithmaticTarget::A)),
            0xC3 => Some(Instruction::Jp()),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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
