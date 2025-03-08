use crate::emulator::flags::FlagsRegister;
use crate::emulator::mmu::Mmu;
use crate::emulator::ppu::Ppu;

const PREFIXED_OPCODE: u8 = 0xCB;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
    ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFFE,
            ime: false,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, _ppu: &mut Ppu) {
        let mut instruction_byte = mmu.read_byte(self.pc);
        let prefixed = instruction_byte == PREFIXED_OPCODE;
        if prefixed {
            instruction_byte = mmu.read_byte(self.pc + 1);
        }

        let instruction = match Instruction::from_byte(instruction_byte, prefixed) {
            Some(instruction) => instruction,
            None => {
                println!("Unknown instruction: {:X}", instruction_byte);
                return;
            }
        };

        println!("Executing instruction: {:?}", instruction);
        self.pc = self.execute_instruction(mmu, instruction);
    }

    fn execute_instruction(&mut self, mmu: &mut Mmu, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Nop() => self.pc.wrapping_add(1),
            Instruction::Halt() => {
                println!("HALT not implemented");
                self.pc.wrapping_add(1)
            }
            Instruction::Load(target) => {
                match target {
                    LoadTarget::SP => {
                        let most_significant_byte = mmu.read_byte(self.pc + 2) as u16;
                        let least_significant_byte = mmu.read_byte(self.pc + 1) as u16;
                        self.sp = (most_significant_byte << 8) | least_significant_byte;
                    }
                }
                self.pc.wrapping_add(3)
            }
            Instruction::Inc(target) => {
                match target {
                    IncTarget::BC => {
                        let value = self.registers.get_bc();
                        self.registers.set_bc(value.wrapping_add(1));
                    }
                    IncTarget::DE => {
                        let value = self.registers.get_de();
                        self.registers.set_de(value.wrapping_add(1));
                    }
                    IncTarget::HL => {
                        let value = self.registers.get_hl();
                        self.registers.set_hl(value.wrapping_add(1));
                    }
                    IncTarget::SP => {
                        self.sp = self.sp.wrapping_add(1);
                    }
                    IncTarget::B => {
                        let value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.registers.f.zero = self.registers.b == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    IncTarget::D => {
                        let value = self.registers.d;
                        self.registers.d = self.registers.d.wrapping_add(1);
                        self.registers.f.zero = self.registers.d == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    IncTarget::H => {
                        let value = self.registers.h;
                        self.registers.h = self.registers.h.wrapping_add(1);
                        self.registers.f.zero = self.registers.h == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::IncHl() => {
                let value = self.registers.get_hl();
                self.registers.set_hl(value.wrapping_add(1));
                self.pc.wrapping_add(1)
            }
            Instruction::Add(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.add(self.registers.a);
                    }
                    ArithmeticTarget::B => {
                        self.add(self.registers.b);
                    }
                    ArithmeticTarget::C => {
                        self.add(self.registers.c);
                    }
                    ArithmeticTarget::D => {
                        self.add(self.registers.d);
                    }
                    ArithmeticTarget::E => {
                        self.add(self.registers.e);
                    }
                    ArithmeticTarget::H => {
                        self.add(self.registers.h);
                    }
                    ArithmeticTarget::L => {
                        self.add(self.registers.l);
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::AddHl() => {
                let hl = self.registers.get_hl();
                let (new_value, did_overflow) = hl.overflowing_add(hl);
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = did_overflow;
                self.registers.f.half_carry = (hl & 0xFFF) + (hl & 0xFFF) > 0xFFF;
                self.registers.set_hl(new_value);
                self.pc.wrapping_add(1)
            }
            Instruction::Jp(test) => {
                let should_jump = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };

                if !should_jump {
                    return self.pc.wrapping_add(3);
                }

                let most_significant_byte = mmu.read_byte(self.pc + 2) as u16;
                let least_significant_byte = mmu.read_byte(self.pc + 1) as u16;
                (most_significant_byte << 8) | least_significant_byte
            }
            Instruction::Jphl() => self.registers.get_hl(),
            Instruction::Rlc(_target) => {
                println!("RLC not implemented");
                self.pc.wrapping_add(1)
            }
            Instruction::Di() => {
                self.ime = false;
                self.pc.wrapping_add(1)
            }
            Instruction::Ei() => {
                self.ime = true;
                self.pc.wrapping_add(1)
            }
        }
    }

    fn add(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.a = new_value;
    }
}

#[derive(Debug)]
enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
enum LoadTarget {
    SP,
}

#[derive(Debug)]
enum IncTarget {
    BC,
    DE,
    HL,
    SP,
    B,
    D,
    H,
}

#[derive(Debug)]
enum Instruction {
    Nop(),
    Inc(IncTarget),
    IncHl(),
    Add(ArithmeticTarget),
    AddHl(),
    Jp(JumpTest),
    Jphl(),
    Halt(),
    Rlc(ArithmeticTarget),
    Di(),
    Ei(),
    Load(LoadTarget),
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
            0x00 => Some(Instruction::Rlc(ArithmeticTarget::B)),
            _ => None,
        }
    }

    fn from_byte_non_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::Nop()),
            0x03 => Some(Instruction::Inc(IncTarget::BC)),
            0x04 => Some(Instruction::Inc(IncTarget::B)),
            0x13 => Some(Instruction::Inc(IncTarget::DE)),
            0x14 => Some(Instruction::Inc(IncTarget::D)),
            0x23 => Some(Instruction::Inc(IncTarget::HL)),
            0x24 => Some(Instruction::Inc(IncTarget::H)),
            0x31 => Some(Instruction::Load(LoadTarget::SP)),
            0x33 => Some(Instruction::Inc(IncTarget::SP)),
            0x34 => Some(Instruction::IncHl()),
            0x76 => Some(Instruction::Halt()),
            0x80 => Some(Instruction::Add(ArithmeticTarget::B)),
            0x81 => Some(Instruction::Add(ArithmeticTarget::C)),
            0x82 => Some(Instruction::Add(ArithmeticTarget::D)),
            0x83 => Some(Instruction::Add(ArithmeticTarget::E)),
            0x84 => Some(Instruction::Add(ArithmeticTarget::H)),
            0x85 => Some(Instruction::Add(ArithmeticTarget::L)),
            0x86 => Some(Instruction::AddHl()),
            0x87 => Some(Instruction::Add(ArithmeticTarget::A)),
            0xC2 => Some(Instruction::Jp(JumpTest::NotZero)),
            0xC3 => Some(Instruction::Jp(JumpTest::Always)),
            0xCA => Some(Instruction::Jp(JumpTest::Zero)),
            0xD2 => Some(Instruction::Jp(JumpTest::NotCarry)),
            0xDA => Some(Instruction::Jp(JumpTest::Carry)),
            0xE9 => Some(Instruction::Jphl()),
            0xF3 => Some(Instruction::Di()),
            0xFB => Some(Instruction::Ei()),
            _ => None,
        }
    }
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
        ((self.b as u16) << 8) | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
