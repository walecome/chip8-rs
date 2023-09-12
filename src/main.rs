pub mod chip8;

use chip8::cpu::Cpu;
use chip8::cpu::Memory;

extern crate sdl2;

use sdl2::audio::AudioDevice;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::AudioSubsystem;

use clap::Parser;
use sdl2::sys::KeyCode;

use std::time::Duration;
use std::time::Instant;

use std::fs;

use sdl2::audio::{AudioCallback, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

struct Beeper {
    device: AudioDevice<SquareWave>,
    is_beeping: bool
}

impl Beeper {
    fn new(audio: AudioSubsystem) -> Beeper {
        let audio_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio
            .open_playback(None, &audio_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 220.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        Beeper {
            device,
            is_beeping: false,
        }
    }

    fn set_beeping(& mut self, beep: bool) {
        if self.is_beeping == beep {
            return;
        }
        if beep {
            self.device.resume();
        } else {
            self.device.pause();
        }
        self.is_beeping = beep;
    }
}

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
    let mut beeper = Beeper::new(sdl_context.audio().unwrap());

    let window = video_subsystem
        .window("Chip-8 emulator", 1200, 600)
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

    let mut sixty_hz_timer = Instant::now();

    let sixty_hz_duration = Duration::from_secs(1) / 60;

    let mut print_timer = Instant::now();
    let print_duration = Duration::from_secs(3);

    let mut frame_times: Vec<Duration> = vec![];
    let mut last_frame_end = Instant::now();

    // TODO: Should we make this configurable?
    let instructions_per_second = 700;
    let cpu_tick_duration = Duration::from_secs(1) / instructions_per_second;
    let mut cpu_timer = Instant::now();

    let mut cpu_tick_times: Vec<Duration> = vec![];
    let mut last_cpu_tick = Instant::now();

    'running: loop {
        if print_timer.elapsed() > print_duration {
            let average_frame_time =
                (&frame_times).into_iter().sum::<Duration>() / (frame_times.len() as u32);
            let frames_per_second =
                Duration::from_secs(1).as_micros() / average_frame_time.as_micros();
            println!(
                "Average {} FPS (frame time: {:?})",
                frames_per_second, average_frame_time
            );

            let average_cpu_tick_time =
                (&cpu_tick_times).into_iter().sum::<Duration>() / (cpu_tick_times.len() as u32);
            let cpu_ticks_per_second =
                Duration::from_secs(1).as_micros() / average_cpu_tick_time.as_micros();
            println!(
                "Average CPU ticks per second {} (tick time: {:?})",
                cpu_ticks_per_second, average_cpu_tick_time
            );

            print_timer = Instant::now();
            frame_times.clear();
            cpu_tick_times.clear();
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
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    cpu.keypad().on_up(scancode);
                }
                _ => {}
            }
        }

        // Tick CPU if needed
        if cpu_timer.elapsed() > cpu_tick_duration {
            let raw_instruction = cpu.fetch();
            let instruction = cpu.decode(raw_instruction);
            cpu.execute(instruction);
            cpu_timer = Instant::now();

            cpu_tick_times.push(last_cpu_tick.elapsed());
            last_cpu_tick = Instant::now();
            beeper.set_beeping(cpu.should_play_sound());
        }

        if sixty_hz_timer.elapsed() < sixty_hz_duration {
            continue;
        }

        // Tick timers
        cpu.tick_timers();
        sixty_hz_timer = Instant::now();

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

        frame_times.push(last_frame_end.elapsed());
        last_frame_end = Instant::now();
    }

    Ok(())
}
