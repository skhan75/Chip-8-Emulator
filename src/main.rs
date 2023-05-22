use std::fs::File;
use std::io::Read;
//
// const MEM_SIZE: usize = 0x1000; // 4096
// const ROM_ADDR: usize = 0x200;
//
// struct Registers([u8; 0x10]);
//
// impl Registers {
//     fn new() -> Self {
//         Self([0; 0x10])
//     }
// }
//
// pub struct Cpu {
//     memory: [u8; MEM_SIZE],     // Memory
//     v: Registers,               // Registers Set
//     i: u16,                     // Index Register
//     pc: u16,                    // Program Counter
//     stack: [u16; 0x10],
//     sp: u8,                     // Stack pointer
//     dt: u8,                     // Delay pointer
// }
//
//
macro_rules! nnn {
     ($w0:expr, $w2:expr) => {
        (($w0 & 0x0f) as u16) << 8 | $w1 as u16
    };
}


// impl Cpu {
//     pub fn new(memory: [u8: MEM_SIZE]) -> Self {
//         Cpu {
//             memory: memory,
//             v: Regs::new(),
//             i: 0,
//             pc: ROM_ADDR as u16,
//             stack: [0; 0x10],
//             sp: 0,
//             dt: 0,
//         }
//     }
//
//
// }
//

pub struct Emulator {
    memory: [u8; 4096],     // 4Kb of RAM
    registers: [u8; 16],    // represents the 16 1-byte registers
    i: u16,                 // represents 16bit Index Register
    stack: [u16, 32],       // The stack offers a max depth of 32 with 2 bytes per stack frame
    stackFrame: i8,         // current stack frame. Initialized as -1 and is set to 0 on first use
    pc: u16,                // Program counter, set it to initial memory offset
    dt: u8,                 // delay timer that is decremented at 60hz if > 0 
    st: u8,                 // sound time that is decremented at 60hz and plays a beep if > 0
}

// Chip-8 interpreter basically works as most other, ie. Fetch  --> Decode --> Execute Cycles

// Pseudo Code 
// -----------
// 1. Load the ROM into `memory`
// 2. Set `pc`ro 0
// 3. Loop until Exit
//  3.1 Read 2 bytes from memory at pc
//      3.1.1 Increment `pc` by 2
//  3.2 Decode those 2 bytes into an `instruction` and possible values
//  3.3 Execute the `instruction`
//  3.4 Repeat



fn main() {
    let mut file = File::open("programs/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();

    file.read_to_end(&mut data);

    print!("Data: {:?}", data);
}
