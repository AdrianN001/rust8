use crate::{cpu::Cpu, graphic::Platform};
use std::time::{Instant, Duration};

mod graphic;
mod cpu;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{

        println!("Usage: <rom/location.ch8>");
        return; 
    }

    let rom_location = &args[1];

    let mut cpu = Cpu::new();
    cpu.load_rom(rom_location);
    let mut platform = Platform::new(1024, 720, "CHIP-8");

    let mut event_pump = platform.sdl_context.event_pump().unwrap();

    let mut last_timer_tick = Instant::now();
    let timer_interval = Duration::from_millis(16); // ~60 Hz

    loop {
        cpu.cycle();
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                return;
            }

            match event {
                sdl2::event::Event::KeyDown { keycode: Some(key), .. } => {
                    cpu.key_pressed(key);
                }

                sdl2::event::Event::KeyUp { keycode: Some(key), .. } => {
                    cpu.key_released(key);
                }

                _ => {}
            }
        }

        if cpu.draw_flag{
            platform.update_frame_buffer(&cpu);
            platform.render();

            cpu.draw_flag = false;
        }

        if last_timer_tick.elapsed() >= timer_interval{
            cpu.tick_timers();
            last_timer_tick = Instant::now();
        }


    }
}

