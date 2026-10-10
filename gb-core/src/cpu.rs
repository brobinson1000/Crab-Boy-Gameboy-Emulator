use crate::registers::Registers;

pub struct MemoryBus {
    memory: [u8; 0x10000], // Layout for memory bus
}

impl MemoryBus {
    pub fn new() -> Self {
        Self { memory: [0; 0x10000] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

pub struct CPU {
    pub registers: Registers,
    pub pc: u16, // Program counter unsigned 16 bit int -> tell which address in memory program is
                 // currently executing on
    pub bus: MemoryBus, // carries data to and from cpu
}

impl CPU {
    fn step(&mut self) {
        let opcode = self.bus.read_byte(self.pc);

        let instruction = Instruction::from_byte(opcode).unwrap_or_else(|| panic!("Unknown OPCODE ERROR {:#40x} {:#60x}", self.pc, opcode));
        self.pc = self.execute(instruction);
    }
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte{
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            _ => None,
        }
    }
}

pub fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
        Instruction::ADD(target) => {
            let value = match target {
                ArithmeticTarget::A => self.registers.a,
                ArithmeticTarget::B => self.registers.b,
                ArithmeticTarget::C => self.registers.c,
                ArithmeticTarget::D => self.registers.d,
                ArithmeticTarget::E => self.registers.e,
                ArithmeticTarget::H => self.registers.h,
                ArithmeticTarget::L => self.registers.l,
            };
            self.registers.a = self.add(value);
            self.pc.wrapping_add(1)
        }
    }
}


