pub mod chip8;

use chip8::cpu::Cpu;
use chip8::cpu::Memory;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

use clap::Parser;
use sdl2::sys::KeyCode;

use std::time::Duration;
use std::time::Instant;

use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    rom: String,
    #[arg(long)]
    use_copy_shift: bool,
    #[arg(long)]
    use_offset_jump_quirk: bool,
}

pub fn main() -> Result<(), String> {
    let args = Args::parse();
    let rom_data = fs::read(args.rom).unwrap();
    let mut cpu = Cpu::new(
        Memory::new(rom_data),
        args.use_copy_shift,
        args.use_offset_jump_quirk,
    );

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip-8 emulator", 1600, 800)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    fn write_pixel(buffer: &mut [u8], pitch: usize, x: usize, y: usize, color: Color) {
        let offset = y * pitch + x * 3;
        buffer[offset] = color.r;
        buffer[offset + 1] = color.g;
        buffer[offset + 2] = color.b
    }

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            cpu.vram().width as u32,
            cpu.vram().height as u32,
        )
        .map_err(|e| e.to_string())?;
    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..cpu.vram().height {
            for x in 0..cpu.vram().width {
                write_pixel(buffer, pitch, x as usize, y as usize, Color::RGB(0, 0, 0));
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = Instant::now();

    let sixty_hz_duration = Duration::from_secs(1) / 60;

    let mut print_timer = Instant::now();
    let print_duration = Duration::from_secs(3);

    let mut frame_times: Vec<Duration> = vec![];

    'running: loop {
        let frame_start = Instant::now();
        if timer.elapsed() > sixty_hz_duration {
            cpu.tick_timers();
            timer = Instant::now();
        }

        if print_timer.elapsed() > print_duration {
            let average_frame_time = (&frame_times).into_iter().sum::<Duration>() / (frame_times.len() as u32);
            let frames_per_second = Duration::from_secs(1).as_micros() / average_frame_time.as_micros();
            println!("Average {} FPS (frame time: {:?})", frames_per_second, average_frame_time);
            print_timer = Instant::now();
            frame_times.clear();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    cpu.keypad().on_down(scancode);
                },
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    cpu.keypad().on_up(scancode);
                },
                _ => {}
            }
        }

        // Tick CPU
        let raw_instruction = cpu.fetch();
        let instruction = cpu.decode(raw_instruction);
        cpu.execute(instruction);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw VRAM
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..cpu.vram().height {
                for x in 0..cpu.vram().width {
                    let color = if cpu.vram().get_cell(x, y) {
                        Color::RGB(255, 255, 255)
                    } else {
                        Color::RGB(0, 0, 0)
                    };
                    write_pixel(buffer, pitch, x as usize, y as usize, color);
                }
            }
        })?;

        canvas.copy(&texture, None, None)?;
        canvas.present();

        frame_times.push(frame_start.elapsed());
    }

    Ok(())
}
