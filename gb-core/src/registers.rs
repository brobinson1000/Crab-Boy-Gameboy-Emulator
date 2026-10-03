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



