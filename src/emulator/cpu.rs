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
                    Target::SP => {
                        let most_significant_byte = mmu.read_byte(self.pc + 2) as u16;
                        let least_significant_byte = mmu.read_byte(self.pc + 1) as u16;
                        self.sp = (most_significant_byte << 8) | least_significant_byte;
                    }
                    _ => panic!("Unimplemented load target"),
                }
                self.pc.wrapping_add(3)
            }
            Instruction::LoadN8(target) => {
                match target {
                    Target::B => {
                        self.registers.b = mmu.read_byte(self.pc + 1);
                    }
                    Target::D => {
                        self.registers.d = mmu.read_byte(self.pc + 1);
                    }
                    Target::H => {
                        self.registers.h = mmu.read_byte(self.pc + 1);
                    }
                    Target::HL => {
                        let val = mmu.read_byte(self.pc + 1);
                        mmu.set_byte(self.registers.get_hl(), val);
                    }
                    Target::C => {
                        self.registers.c = mmu.read_byte(self.pc + 1);
                    }
                    Target::E => {
                        self.registers.e = mmu.read_byte(self.pc + 1);
                    }
                    Target::L => {
                        self.registers.l = mmu.read_byte(self.pc + 1);
                    }
                    Target::A => {
                        self.registers.a = mmu.read_byte(self.pc + 1);
                    }
                    _ => panic!("Unimplemented loadN8 target"),
                }
                self.pc.wrapping_add(2)
            }
            Instruction::LoadHC() => {
                mmu.set_byte(0xFF00 + self.registers.c as u16, self.registers.a);
                self.pc.wrapping_add(1)
            }
            Instruction::LoadHA() => {
                self.registers.a = mmu.read_byte(0xFF00 + self.registers.c as u16);
                self.pc.wrapping_add(1)
            }
            Instruction::Inc(target) => {
                match target {
                    Target::BC => {
                        let value = self.registers.get_bc();
                        self.registers.set_bc(value.wrapping_add(1));
                    }
                    Target::DE => {
                        let value = self.registers.get_de();
                        self.registers.set_de(value.wrapping_add(1));
                    }
                    Target::HL => {
                        let value = self.registers.get_hl();
                        self.registers.set_hl(value.wrapping_add(1));
                    }
                    Target::SP => {
                        self.sp = self.sp.wrapping_add(1);
                    }
                    Target::A => {
                        let value = self.registers.a;
                        self.registers.a = self.registers.a.wrapping_add(1);
                        self.registers.f.zero = self.registers.a == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    Target::B => {
                        let value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.registers.f.zero = self.registers.b == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    Target::D => {
                        let value = self.registers.d;
                        self.registers.d = self.registers.d.wrapping_add(1);
                        self.registers.f.zero = self.registers.d == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    Target::H => {
                        let value = self.registers.h;
                        self.registers.h = self.registers.h.wrapping_add(1);
                        self.registers.f.zero = self.registers.h == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
                    }
                    _ => panic!("Unimplemented inc target"),
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
                    Target::A => {
                        self.add(self.registers.a);
                    }
                    Target::B => {
                        self.add(self.registers.b);
                    }
                    Target::C => {
                        self.add(self.registers.c);
                    }
                    Target::D => {
                        self.add(self.registers.d);
                    }
                    Target::E => {
                        self.add(self.registers.e);
                    }
                    Target::H => {
                        self.add(self.registers.h);
                    }
                    Target::L => {
                        self.add(self.registers.l);
                    }
                    _ => panic!("Unimplemented add target"),
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
            Instruction::Jump(test) => {
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
            Instruction::JumpHl() => self.registers.get_hl(),
            Instruction::RotateLeftCircular(target) => {
                match target {
                    Target::A => {
                        let msb = (self.registers.a & 0b1000_0000) >> 7;
                        self.registers.a = (self.registers.a << 1) | msb;
                        self.registers.f.carry = msb == 1;
                    }
                    Target::B => {
                        let msb = (self.registers.b & 0b1000_0000) >> 7;
                        self.registers.b = (self.registers.b << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.b == 0;
                    }
                    Target::C => {
                        let msb = (self.registers.c & 0b1000_0000) >> 7;
                        self.registers.c = (self.registers.c << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.c == 0;
                    }
                    Target::D => {
                        let msb = (self.registers.d & 0b1000_0000) >> 7;
                        self.registers.d = (self.registers.d << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.d == 0;
                    }
                    Target::E => {
                        let msb = (self.registers.e & 0b1000_0000) >> 7;
                        self.registers.e = (self.registers.e << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.e == 0;
                    }
                    Target::H => {
                        let msb = (self.registers.h & 0b1000_0000) >> 7;
                        self.registers.h = (self.registers.h << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.h == 0;
                    }
                    Target::L => {
                        let msb = (self.registers.l & 0b1000_0000) >> 7;
                        self.registers.l = (self.registers.l << 1) | msb;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.l == 0;
                    }
                    _ => panic!("Unimplemented rotate left circular target"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::RotateRightCircular(target) => {
                match target {
                    Target::A => {
                        let lsb = self.registers.a & 0b0000_0001;
                        self.registers.a >>= 1;
                        self.registers.a |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                    }
                    Target::B => {
                        let lsb = self.registers.b & 0b0000_0001;
                        self.registers.b >>= 1;
                        self.registers.b |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.b == 0;
                    }
                    Target::C => {
                        let lsb = self.registers.c & 0b0000_0001;
                        self.registers.c >>= 1;
                        self.registers.c |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.c == 0;
                    }
                    Target::D => {
                        let lsb = self.registers.d & 0b0000_0001;
                        self.registers.d >>= 1;
                        self.registers.d |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.d == 0;
                    }
                    Target::E => {
                        let lsb = self.registers.e & 0b0000_0001;
                        self.registers.e >>= 1;
                        self.registers.e |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.e == 0;
                    }
                    Target::H => {
                        let lsb = self.registers.h & 0b0000_0001;
                        self.registers.h >>= 1;
                        self.registers.h |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.h == 0;
                    }
                    Target::L => {
                        let lsb = self.registers.l & 0b0000_0001;
                        self.registers.l >>= 1;
                        self.registers.l |= lsb << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.l == 0;
                    }
                    _ => panic!("Unimplemented rotate right circular target"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::RotateLeft(target) => {
                match target {
                    Target::A => {
                        let msb = (self.registers.a & 0b1000_0000) >> 7;
                        self.registers.a = (self.registers.a << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                    }
                    Target::B => {
                        let msb = (self.registers.b & 0b1000_0000) >> 7;
                        self.registers.b = (self.registers.b << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.b == 0;
                    }
                    Target::C => {
                        let msb = (self.registers.c & 0b1000_0000) >> 7;
                        self.registers.c = (self.registers.c << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.c == 0;
                    }
                    Target::D => {
                        let msb = (self.registers.d & 0b1000_0000) >> 7;
                        self.registers.d = (self.registers.d << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.d == 0;
                    }
                    Target::E => {
                        let msb = (self.registers.e & 0b1000_0000) >> 7;
                        self.registers.e = (self.registers.e << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.e == 0;
                    }
                    Target::H => {
                        let msb = (self.registers.h & 0b1000_0000) >> 7;
                        self.registers.h = (self.registers.h << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.h == 0;
                    }
                    Target::L => {
                        let msb = (self.registers.l & 0b1000_0000) >> 7;
                        self.registers.l = (self.registers.l << 1) | self.registers.f.carry as u8;
                        self.registers.f.carry = msb == 1;
                        self.registers.f.zero = self.registers.l == 0;
                    }
                    _ => panic!("Unimplemented rotate left target"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::RotateRight(target) => {
                match target {
                    Target::A => {
                        let lsb = self.registers.a & 0b0000_0001;
                        self.registers.a >>= 1;
                        self.registers.a |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                    }
                    Target::B => {
                        let lsb = self.registers.b & 0b0000_0001;
                        self.registers.b >>= 1;
                        self.registers.b |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.b == 0;
                    }
                    Target::C => {
                        let lsb = self.registers.c & 0b0000_0001;
                        self.registers.c >>= 1;
                        self.registers.c |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.c == 0;
                    }
                    Target::D => {
                        let lsb = self.registers.d & 0b0000_0001;
                        self.registers.d >>= 1;
                        self.registers.d |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.d == 0;
                    }
                    Target::E => {
                        let lsb = self.registers.e & 0b0000_0001;
                        self.registers.e >>= 1;
                        self.registers.e |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.e == 0;
                    }
                    Target::H => {
                        let lsb = self.registers.h & 0b0000_0001;
                        self.registers.h >>= 1;
                        self.registers.h |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.h == 0;
                    }
                    Target::L => {
                        let lsb = self.registers.l & 0b0000_0001;
                        self.registers.l >>= 1;
                        self.registers.l |= (self.registers.f.carry as u8) << 7;
                        self.registers.f.carry = lsb == 1;
                        self.registers.f.zero = self.registers.l == 0;
                    }
                    _ => panic!("Unimplemented rotate right target"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::LoadIntoMemory() => {
                let low = mmu.read_byte(self.pc + 1) as u16;
                let high = mmu.read_byte(self.pc + 2) as u16;
                let address = (high << 8) | low;
                mmu.set_byte(address, self.registers.a);
                self.pc.wrapping_add(3)
            }
            Instruction::DisableInterrupt() => {
                self.ime = false;
                self.pc.wrapping_add(1)
            }
            Instruction::EnableInterrupts() => {
                self.ime = true;
                self.pc.wrapping_add(1)
            }
            Instruction::Restart(address) => {
                self.sp = self.sp.wrapping_sub(1);
                mmu.set_byte(self.sp, (self.pc >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                mmu.set_byte(self.sp, (self.pc & 0xFF) as u8);
                address
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
enum Target {
    BC,
    DE,
    HL,
    SP,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
enum Instruction {
    Nop(),
    Inc(Target),
    IncHl(),
    Add(Target),
    AddHl(),
    Jump(JumpTest),
    JumpHl(),
    Halt(),
    RotateLeftCircular(Target),
    RotateRightCircular(Target),
    RotateLeft(Target),
    RotateRight(Target),
    DisableInterrupt(),
    EnableInterrupts(),
    Load(Target),
    LoadIntoMemory(),
    LoadN8(Target),
    LoadHC(),
    LoadHA(),
    Restart(u16),
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
            0x00 => Some(Instruction::RotateLeftCircular(Target::B)),
            0x01 => Some(Instruction::RotateLeftCircular(Target::C)),
            0x02 => Some(Instruction::RotateLeftCircular(Target::D)),
            0x03 => Some(Instruction::RotateLeftCircular(Target::E)),
            0x04 => Some(Instruction::RotateLeftCircular(Target::H)),
            0x05 => Some(Instruction::RotateLeftCircular(Target::L)),
            0x07 => Some(Instruction::RotateLeftCircular(Target::A)),
            0x08 => Some(Instruction::RotateRightCircular(Target::B)),
            0x09 => Some(Instruction::RotateRightCircular(Target::C)),
            0x0A => Some(Instruction::RotateRightCircular(Target::D)),
            0x0B => Some(Instruction::RotateRightCircular(Target::E)),
            0x0C => Some(Instruction::RotateRightCircular(Target::H)),
            0x0D => Some(Instruction::RotateRightCircular(Target::L)),
            0x0F => Some(Instruction::RotateRightCircular(Target::A)),
            0x10 => Some(Instruction::RotateLeft(Target::B)),
            0x11 => Some(Instruction::RotateLeft(Target::C)),
            0x12 => Some(Instruction::RotateLeft(Target::D)),
            0x13 => Some(Instruction::RotateLeft(Target::E)),
            0x14 => Some(Instruction::RotateLeft(Target::H)),
            0x15 => Some(Instruction::RotateLeft(Target::L)),
            0x17 => Some(Instruction::RotateLeft(Target::A)),
            0x18 => Some(Instruction::RotateRight(Target::B)),
            0x19 => Some(Instruction::RotateRight(Target::C)),
            0x1A => Some(Instruction::RotateRight(Target::D)),
            0x1B => Some(Instruction::RotateRight(Target::E)),
            0x1C => Some(Instruction::RotateRight(Target::H)),
            0x1D => Some(Instruction::RotateRight(Target::L)),
            0x1F => Some(Instruction::RotateRight(Target::A)),
            _ => None,
        }
    }

    fn from_byte_non_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::Nop()),
            0x03 => Some(Instruction::Inc(Target::BC)),
            0x04 => Some(Instruction::Inc(Target::B)),
            0x06 => Some(Instruction::LoadN8(Target::B)),
            0x07 => Some(Instruction::RotateLeft(Target::A)),
            0x0E => Some(Instruction::LoadN8(Target::C)),
            0x0F => Some(Instruction::RotateRight(Target::A)),
            0x13 => Some(Instruction::Inc(Target::DE)),
            0x14 => Some(Instruction::Inc(Target::D)),
            0x16 => Some(Instruction::LoadN8(Target::D)),
            0x1E => Some(Instruction::LoadN8(Target::E)),
            0x23 => Some(Instruction::Inc(Target::HL)),
            0x24 => Some(Instruction::Inc(Target::H)),
            0x26 => Some(Instruction::LoadN8(Target::H)),
            0x2E => Some(Instruction::LoadN8(Target::L)),
            0x31 => Some(Instruction::Load(Target::SP)),
            0x33 => Some(Instruction::Inc(Target::SP)),
            0x34 => Some(Instruction::IncHl()),
            0x36 => Some(Instruction::LoadN8(Target::HL)),
            0x3C => Some(Instruction::Inc(Target::A)),
            0x3E => Some(Instruction::LoadN8(Target::A)),
            0x76 => Some(Instruction::Halt()),
            0x80 => Some(Instruction::Add(Target::B)),
            0x81 => Some(Instruction::Add(Target::C)),
            0x82 => Some(Instruction::Add(Target::D)),
            0x83 => Some(Instruction::Add(Target::E)),
            0x84 => Some(Instruction::Add(Target::H)),
            0x85 => Some(Instruction::Add(Target::L)),
            0x86 => Some(Instruction::AddHl()),
            0x87 => Some(Instruction::Add(Target::A)),
            0xC2 => Some(Instruction::Jump(JumpTest::NotZero)),
            0xC3 => Some(Instruction::Jump(JumpTest::Always)),
            0xCA => Some(Instruction::Jump(JumpTest::Zero)),
            0xD2 => Some(Instruction::Jump(JumpTest::NotCarry)),
            0xDA => Some(Instruction::Jump(JumpTest::Carry)),
            0xE0 => Some(Instruction::LoadHC()),
            0xEA => Some(Instruction::LoadIntoMemory()),
            0xE9 => Some(Instruction::JumpHl()),
            0xF0 => Some(Instruction::LoadHA()),
            0xF3 => Some(Instruction::DisableInterrupt()),
            0xFB => Some(Instruction::EnableInterrupts()),
            0xFF => Some(Instruction::Restart(0x38)),
            _ => None,
        }
    }
}

pub struct Registers {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rotate_right_circular() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b0000_0001;
        cpu.execute_instruction(&mut Mmu::new(), Instruction::RotateRightCircular(Target::B));
        assert_eq!(cpu.registers.b, 0b1000_0000);
        assert!(!cpu.registers.f.zero);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_rotate_left_circular() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0000;
        cpu.execute_instruction(&mut Mmu::new(), Instruction::RotateLeftCircular(Target::B));
        assert_eq!(cpu.registers.b, 0b0000_0001);
        assert!(!cpu.registers.f.zero);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_rotate_right() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b0000_0001;
        cpu.execute_instruction(&mut Mmu::new(), Instruction::RotateRight(Target::B));
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert!(cpu.registers.f.zero);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_rotate_left() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0000;
        cpu.execute_instruction(&mut Mmu::new(), Instruction::RotateLeft(Target::B));
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert!(cpu.registers.f.zero);
        assert!(cpu.registers.f.carry);
    }
}
