use crate::{cpu::Cpu, graphic::Platform};

mod graphic;
mod cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_rom("rom/ibm.ch8".to_string());
    let mut platform = Platform::new(1024, 720, "CHIP-8");

    let mut event_pump = platform.sdl_context.event_pump().unwrap();

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

        platform.update_frame_buffer(&cpu);
        platform.render();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

