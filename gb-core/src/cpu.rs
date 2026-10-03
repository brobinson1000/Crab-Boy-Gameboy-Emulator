use crate::registers::Registers;

enum Instruction {
    ADD(Arithmetic_Target),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub struct CPU {
    registers::Registers,
    pc: u16, // Program Counter
    bus: MemoryBus,
}

// MemoryBus to carry data between RAM and CPU
struct MemoryBus {
    memory: [u8, 0x10000], // size of 64KiB
}



