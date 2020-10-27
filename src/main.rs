struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
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

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                // Opcode x0000 means stop.
                (0, 0, 0, 0) => return,
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
}

fn main() {
    // let opcode: u16 = 0x71E4;
    //
    // let c = ((opcode & 0xF000) >> 12) as u8;
    // let x = ((opcode & 0x0F00) >> 8) as u8;
    // let y = ((opcode & 0x00F0) >> 4) as u8;
    // let d = ((opcode & 0x000F) >> 0) as u8;
    //
    // assert_eq!(c, 0x7);
    // assert_eq!(x, 0x1);
    // assert_eq!(y, 0xE);
    // assert_eq!(d, 0x4);
    //
    // let nnn = opcode & 0x0FFF;
    // let kk = opcode & 0x00FF;
    //
    // assert_eq!(nnn, 0x1E4);
    // assert_eq!(kk, 0xE4);

    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    // 0x8014 - add register 1 to register 0.
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;

    // 0x8024 - add register 2 to register 0.
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;

    // 0x8034 - add register 3 to register 0.
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
