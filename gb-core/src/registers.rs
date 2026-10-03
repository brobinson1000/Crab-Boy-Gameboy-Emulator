// Gameboy CPU 8 - 8 bit registers
struct Registers {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
}

// Gets 16 bit combined regsiter 
impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value : u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = ((value & 0xFF00) as u8;
    }
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}


const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (flag.zero as u8) << ZERO_FLAG_BYTE_POSITION
            | (flag.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION
            | (flag.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION
            | (flag.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        Self {
            zero: (byte >> ZERO_FLAG_BYTE_POSITION) & 1 != 0,
            subtract: (byte >> SUBTRACT_FLAG_BYTE_POSITION) & 1 != 0,
            half_carry: (byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 1 != 0,
            carry: (byte >> CARRY_FLAG_BYTE_POSITION) & 1 != 0,
        }
    }
}

enum Instruction {
    ADD(Arithmetic_Target),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

impl CPU {
    fn execute(&mut self, instruction::Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }

                }
            }
        }
    }
    fn add(&mut self, value : u8) -> u8 {
        let (new_value, did_overflow) = self.register.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    
}


