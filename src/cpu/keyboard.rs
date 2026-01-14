use sdl2::keyboard::Keycode;



impl super::Cpu{
    pub fn key_pressed(&mut self, key: Keycode){

        match key {
            Keycode::Num1 => self.input_keys[0x1_usize] = true,
            Keycode::Num2 => self.input_keys[0x2_usize] = true,
            Keycode::Num3 => self.input_keys[0x3_usize] = true,
            Keycode::Num4 => self.input_keys[0xC_usize] = true,
            Keycode::Q    => self.input_keys[0x4_usize] = true,
            Keycode::W    => self.input_keys[0x5_usize] = true,
            Keycode::E    => self.input_keys[0x6_usize] = true,
            Keycode::R    => self.input_keys[0xD_usize] = true,
            Keycode::A    => self.input_keys[0x7_usize] = true,
            Keycode::S    => self.input_keys[0x8_usize] = true,
            Keycode::D    => self.input_keys[0x9_usize] = true,
            Keycode::F    => self.input_keys[0xE_usize] = true,
            Keycode::Z    => self.input_keys[0xA_usize] = true,
            Keycode::X    => self.input_keys[0x0_usize] = true,
            Keycode::C    => self.input_keys[0xB_usize] = true,
            Keycode::V    => self.input_keys[0xF_usize] = true,
            _ => {
                
            }
        }
    }

    pub fn key_released(&mut self, key: Keycode){

        match key {
            Keycode::Num1 => self.input_keys[0x1_usize] = false,
            Keycode::Num2 => self.input_keys[0x2_usize] = false,
            Keycode::Num3 => self.input_keys[0x3_usize] = false,
            Keycode::Num4 => self.input_keys[0xC_usize] = false,
            Keycode::Q    => self.input_keys[0x4_usize] = false,
            Keycode::W    => self.input_keys[0x5_usize] = false,
            Keycode::E    => self.input_keys[0x6_usize] = false,
            Keycode::R    => self.input_keys[0xD_usize] = false,
            Keycode::A    => self.input_keys[0x7_usize] = false,
            Keycode::S    => self.input_keys[0x8_usize] = false,
            Keycode::D    => self.input_keys[0x9_usize] = false,
            Keycode::F    => self.input_keys[0xE_usize] = false,
            Keycode::Z    => self.input_keys[0xA_usize] = false,
            Keycode::X    => self.input_keys[0x0_usize] = false,
            Keycode::C    => self.input_keys[0xB_usize] = false,
            Keycode::V    => self.input_keys[0xF_usize] = false,
            _ => {
                
            }
        }

    }
}
