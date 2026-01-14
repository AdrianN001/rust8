extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use crate::cpu::Cpu;

pub struct Platform{
    pub sdl_context:    Sdl,
    video:          VideoSubsystem,

    canvas:         Canvas<Window>,
    frame_buffer:   Vec<u32>,

    window_height:  i32,
    window_width:   i32
}


impl Platform{

    const CHIP8_VRAM_WIDTH: u32 = 64;
    const CHIP8_VRAM_HEIGHT:u32 = 32;

    const OFF_COLOR:u32 = 0xFF023612;
    const ON_COLOR: u32 = 0xFF1FC742;

    pub fn new(width: i32, height: i32, title: &str) -> Self{
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window(title, width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .accelerated()
            .build()
            .unwrap();

        Self{
            sdl_context: sdl, 
            video, 

            canvas, 
            frame_buffer: vec![0u32; (Self::CHIP8_VRAM_HEIGHT*Self::CHIP8_VRAM_WIDTH) as usize],

            window_width: width,
            window_height: height
        }
    }

    pub fn render(&mut self){
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::ARGB8888,
                Self::CHIP8_VRAM_WIDTH as u32,
                Self::CHIP8_VRAM_HEIGHT as u32,
            ).unwrap();

        
        texture.update(
                None,
                bytemuck::cast_slice(&self.frame_buffer),
            Self::CHIP8_VRAM_WIDTH as usize * std::mem::size_of::<u32>()

        ).unwrap();


        self.canvas.clear();

        self.canvas
            .copy(
                &texture,
                None,
                Some(sdl2::rect::Rect::new(
                    0,
                    0,
                    self.window_width as u32,
                    self.window_height as u32
            )),
        ).unwrap();

        self.canvas.present();

    }

    pub fn update_frame_buffer(&mut self, cpu: &Cpu){
        for i in 0..(Self::CHIP8_VRAM_WIDTH*Self::CHIP8_VRAM_HEIGHT){
            self.frame_buffer[i as usize] = if cpu.is_pixel_on_at(i as usize){ Self::ON_COLOR } else { Self::OFF_COLOR };
        } 
    }

}
