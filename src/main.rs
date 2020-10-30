struct CPU {
    memory: [u8; 0x1000],

    // After 16 function calls we will get stack overflow.
    stack: [u16; 16],
    stack_pointer: usize,

    // 0 to F numbers can address 16 registers.
    registers: [u8; 16],

    // Program counter.
    position_in_memory: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // To create a u16 opcode we combine two values from memory
        // with the logical OR operation.
        // They need to be cast as u16 to start with, otherwise the left-shift
        // will set all the bits to 0.
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            // Increment position in memory to point to the next instruction.
            self.position_in_memory += 2;

            // Read an opcode group.
            let c = ((opcode & 0xF000) >> 12) as u8;

            // Read the first register.
            let x = ((opcode & 0x0F00) >> 8) as u8;

            // Read the second register.
            let y = ((opcode & 0x00F0) >> 4) as u8;

            // Read an opcode subgroup.
            let d = ((opcode & 0x000F) >> 0) as u8;

            // Read a memory address.
            let nnn = opcode & 0x0FFF;

            // Read an integer.
            // let kk  = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;

            return;
        }

        self.registers[0xF] = 0;
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
