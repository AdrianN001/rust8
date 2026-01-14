use rand::Rng;

//
// https://austinmorlan.com/posts/chip8_emulator/
//
impl super::Cpu {
    
    // 00E0: CLS 
    pub fn op_00e0(&mut self){
        self.vram = [false; 64*32];
    }

    // 00EE: RET
    pub fn op_00ee(&mut self){
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    // 1nnn: JP addr
    pub fn op_1nnn(&mut self){
        let address: u16 = self.opcode & 0x0FFFu16;

        self.program_counter = address;
    }

    // 2nnn - CALL addr
    pub fn op_2nnn(&mut self){
        let address: u16 = self.opcode & 0x0FFFu16;

        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = address;
    }

    // 3xkk - SE Vx, byte
    pub fn op_3xkk(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[Vx as usize] == byte{
            self.program_counter += 2;
        }
    }

    // 4xkk - SNE Vx, byte 
    pub fn op_4xkk(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[Vx as usize] != byte{
            self.program_counter += 2;
        }
    }

    // 5xy0 - SE Vx, Vy
    pub fn op_5xy0(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
       
        if self.registers[Vx as usize] == self.registers[Vy as usize]{
            self.program_counter += 2;
        }
    }

    // 6xkk - LD Vx, byte
    pub fn op_6xkk(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[Vx as usize] = byte;
    }

    // 7xkk - ADD Vx, byte
    pub fn op_7xkk(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[Vx as usize] += byte;
    }
    
    // 8xy0 - LD Vx, Vy
    pub fn op_8xy0(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
       
        self.registers[Vx as usize] = self.registers[Vy as usize];
    }

    // 8xy1 - OR Vx, Vy
    pub fn op_8xy1(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
       
        self.registers[Vx as usize] |= self.registers[Vy as usize];
    }

    // 8xy2 - AND Vx, Vy
    pub fn op_8xy2(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
       
        self.registers[Vx as usize] &= self.registers[Vy as usize];
    }

    // 8xy3 - XOR Vx, Vy
    pub fn op_8xy3(&mut self){
        let Vx: u8 = ((self.opcode & 0x0f00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
       
        self.registers[Vx as usize] ^= self.registers[Vy as usize];
    }

    // 8xy4 - ADD Vx, Vy
    pub fn op_8xy4(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;

        let sum: u16 = self.registers[Vx as usize] as u16 + self.registers[Vy as usize] as u16;

        if sum > 255u16{
            self.registers[0xF_usize] = 1;
        }else{
            self.registers[0xF_usize] = 0; 
        }

        self.registers[Vx as usize] = (sum & 0xFFu16) as u8;
    }

    // 8xy5 - SUB Vx, Vy 
    pub fn op_8xy5(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;
        
        if self.registers[Vx as usize] > self.registers[Vy as usize]{
            self.registers[0xF_usize] = 1;
        }else{
            self.registers[0xF_usize] = 0;
        }
        self.registers[Vx as usize] -= self.registers[Vy as usize];
    }

    // 8xy6 - SHR Vx
    pub fn op_8xy6(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        
        self.registers[0xF_usize] = self.registers[Vx as usize] & 0x01u8;
        
        self.registers[Vx as usize] >>= 1;
    }

    // 8xy7 - SUBN Vx, Vy
    pub fn op_8xy7(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x0f0) >> 4u8) as u8;


        if self.registers[Vy as usize] > self.registers[Vx as usize]{
            self.registers[0xF_usize] = 1;
        }else{
            self.registers[0xF_usize] = 0; 
        }

        self.registers[Vx as usize] = self.registers[Vy as usize] - self.registers[Vx as usize];
    }

    // 8xyE - SHL Vx {, Vy}
    pub fn op_8xye(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        
        self.registers[0xF_usize] = (self.registers[Vx as usize] & 0x80u8) >> 7u8;
        
        self.registers[Vx as usize] <<= 1;

    }

    // 9xy0 - SNE Vx, Vy
    pub fn op_9xy0(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        if self.registers[Vx as usize] != self.registers[Vy as usize]{
            self.program_counter += 2;
        }
    }

    // Annn - LD I, addr 
    pub fn op_annn(&mut self){
        let address: u16 = self.opcode & 0x0FFF;

        self.index_register = address;
    }

    // Bnnn - JP V0, addr 
    pub fn op_bnnn(&mut self){
        let address: u16 = self.opcode & 0x0FF0;

        self.program_counter = self.registers[0] as u16 + address;
    }

    // Cxkk - RND Vx, byte 
    pub fn op_cxkk(&mut self){
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;
       

        let rnd: u8 = self.rng_engine.random_range(0..=255);
        self.registers[Vx as usize] = rnd & byte;
    }

    // Dxyn - DRW Vx, Vy, nibble
    pub fn op_dxyn(&mut self){
        const VIDEO_WIDTH: u8 = 64;
        const VIDEO_HEIGHT:u8 = 32;

        self.draw_flag = true;


        let Vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8; 
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        let height: u8 = (self.opcode & 0x000F) as u8;

        let x_pos: u8 = self.registers[Vx as usize] % VIDEO_WIDTH;
        let y_pos: u8 = self.registers[Vy as usize] % VIDEO_HEIGHT;

        self.registers[0xF_usize] = 0;

        for row in 0..(height as u64){
            let sprite_byte = self.memory[(self.index_register as u64 + row) as usize];

            for col in 0..8u64{
                let sprite_pixel = (sprite_byte & (0x80 >> col)) != 0;

                let x = (x_pos as u64 + col) % 64;
                let y = (y_pos as u64 + row) % 32;
                let index = y * 64 + x;

                if sprite_pixel {
                    // Collision
                    if self.vram[index as usize] {
                        self.registers[0xF] = 1;
                    }

                    // XOR draw
                    self.vram[index as usize] ^= true;
                }
            }
        }
    }


    // Ex9E - SKP Vx
    pub fn op_ex9e(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: usize = self.registers[Vx as usize] as usize;

        if self.input_keys[key] {
            self.program_counter += 2;
        }
    }

        
    // ExA1 - SKNP Vx
    pub fn op_exa1(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: usize = self.registers[Vx as usize] as usize;

        if !self.input_keys[key] {
            self.program_counter += 2;
        }
    }

    // Fx07 - LD Vx, DT
    pub fn op_fx07(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[Vx as usize] = self.delay_timer;
    }

    // Fx0A - LD Vx, K
    pub fn op_fx0a(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for key in 0..16 {
            if self.input_keys[key] {
                self.registers[Vx as usize] = key as u8;
                return;
            }
        }

        // No key pressed → repeat instruction
        self.program_counter -= 2;
    }

    // Fx15 - LD DT, Vx
    pub fn op_fx15(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.delay_timer = self.registers[Vx as usize];
    }

    // Fx18 - LD ST, Vx
    pub fn op_fx18(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.sound_timer = self.registers[Vx as usize];
    }

    // Fx1E - ADD I, Vx
    pub fn op_fx1e(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.index_register += self.registers[Vx as usize] as u16;
    }

    // Fx29 - LD F, Vx
    pub fn op_fx29(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit: u8 = self.registers[Vx as usize];

        self.index_register = super::font::FONTSET_START_ADDRESS as u16 + (5 * digit as u16);
    }

    // Fx33 - LD B, Vx
    pub fn op_fx33(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let mut value: u8 = self.registers[Vx as usize];

        let i = self.index_register as usize;

        // Ones
        self.memory[i + 2] = value % 10;
        value /= 10;

        // Tens
        self.memory[i + 1] = value % 10;
        value /= 10;

        // Hundreds
        self.memory[i] = value % 10;
    }

    // Fx55 - LD [I], Vx
    pub fn op_fx55(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let i = self.index_register as usize;

        for reg in 0..=Vx {
            self.memory[i + reg as usize] = self.registers[reg as usize];
        }
    }

    // Fx65 - LD Vx, [I]
    pub fn op_fx65(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let i = self.index_register as usize;

        for reg in 0..=Vx {
            self.registers[reg as usize] = self.memory[i + reg as usize];
        }
    }
}

