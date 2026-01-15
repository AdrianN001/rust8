use std::{fs::File, io::{BufReader, Read}};
use rand::{Rng, rngs::ThreadRng};

use crate::cpu::font::load_fonts_into_memory;

mod font;
mod instruction;
mod keyboard;

const START_ADDRESS: u16 = 0x200;

pub struct Cpu{
    registers:      [u8; 16],
    memory:         [u8; 4*1024],

    index_register: u16,
    program_counter:u16,

    stack:          [u16; 16],
    stack_pointer:  u8,

    delay_timer:    u8,
    sound_timer:    u8,

    input_keys:     [bool; 16],

    vram:           [bool; 64*32],
    rng_engine:     ThreadRng,
    opcode:         u16,

    pub draw_flag:  bool
}

impl Cpu{
            
    pub fn new() -> Self{
        let mut new_cpu = Self {
            registers: [0; 16],
            memory: [0; 4 * 1024],

            index_register: 0,
            program_counter: START_ADDRESS, // typisch z.B. bei CHIP-8

            stack: [0; 16],
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,

            input_keys: [false; 16],

            vram: [false; 64 * 32],
            
            rng_engine: rand::rng(),

            opcode: 0,
            draw_flag: false
        };
            
        load_fonts_into_memory(&mut new_cpu);

        new_cpu
    }


    pub fn load_rom(&mut self, file_name: &String){
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        
        for (index, byte_or_error) in reader.bytes().enumerate(){
            let byte = byte_or_error.unwrap();
            let insert_index = START_ADDRESS as usize + index;
            self.memory[insert_index] = byte;
        }
    }

    pub fn cycle(&mut self){

        self.opcode =
            (self.memory[self.program_counter as usize] as u16) << 8
            | (self.memory[self.program_counter as usize + 1] as u16);

        self.program_counter += 2;

        println!("PC={:#05X} OPCODE={:#06X}", self.program_counter, self.opcode);

        // -- Dispatch -- 
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode {
                0x00E0 => self.op_00e0(), // CLS
                0x00EE => self.op_00ee(), // RET
                _ => {} // 0NNN ignored
            },

            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => {
                if self.opcode & 0x000F == 0 {
                    self.op_5xy0();
                }
            }

            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),

            0x8000 => match self.opcode & 0x000F {
                0x0 => self.op_8xy0(),
                0x1 => self.op_8xy1(),
                0x2 => self.op_8xy2(),
                0x3 => self.op_8xy3(),
                0x4 => self.op_8xy4(),
                0x5 => self.op_8xy5(),
                0x6 => self.op_8xy6(),
                0x7 => self.op_8xy7(),
                0xE => self.op_8xye(),
                _ => {}
            },

            0x9000 => {
                if self.opcode & 0x000F == 0 {
                    self.op_9xy0();
                }
            }

            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxkk(),
            0xD000 => self.op_dxyn(),

            0xE000 => match self.opcode & 0x00FF {
                0x9E => self.op_ex9e(),
                0xA1 => self.op_exa1(),
                _ => {}
            },

            0xF000 => match self.opcode & 0x00FF {
                0x07 => self.op_fx07(),
                0x0A => self.op_fx0a(),
                0x15 => self.op_fx15(),
                0x18 => self.op_fx18(),
                0x1E => self.op_fx1e(),
                0x29 => self.op_fx29(),
                0x33 => self.op_fx33(),
                0x55 => self.op_fx55(),
                0x65 => self.op_fx65(),
                _ => {}
            },

            _ => {
                // Optional Debug
                // println!("Unknown opcode: {:#06X}", self.opcode);
            }
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn is_pixel_on_at(&self, indx: usize) -> bool{
        self.vram[indx]
    }

    
}

