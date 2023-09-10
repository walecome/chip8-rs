pub mod chip8;

use chip8::cpu::Cpu;
use chip8::cpu::Memory;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use clap::Parser;

use std::time::Duration;

use std::fs;

struct GridMapper {
    cell_size: u32,
    width: u32,
    height: u32,
    divider_size: u32,
}

enum GridPixelType {
    Divider,
    Cell { vram_x: u8, vram_y: u8 },
}

impl GridMapper {
    fn new(grid_width: u32, grid_height: u32, cell_size: u32) -> GridMapper {
        let divider_size: u32 = 1;
        let width = (grid_width * cell_size) + (divider_size * (grid_width - 1));
        let height = (grid_height * cell_size) + (divider_size * (grid_height - 1));

        return GridMapper {
            cell_size,
            width,
            height,
            divider_size,
        };
    }

    fn get_type(&self, x: u32, y: u32) -> GridPixelType {
        let chunk_size = self.cell_size + self.divider_size;
        let x_hits_divider = x % chunk_size == 0;
        let y_hits_divider = y % chunk_size == 0;

        return if x_hits_divider || y_hits_divider {
            GridPixelType::Divider
        } else {
            GridPixelType::Cell {
                vram_x: (x / chunk_size) as u8,
                vram_y: (y / chunk_size) as u8,
            }
        };
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    rom: String,
    #[arg(long)]
    use_copy_shift: bool,
}

fn main() {
    let args = Args::parse();
    let rom_data = fs::read(args.rom).unwrap();

    let mut cpu = Cpu::new(Memory::new(rom_data), args.use_copy_shift);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let grid_mapper = GridMapper::new(cpu.vram().width as u32, cpu.vram().height as u32, 15);
    let window = video_subsystem
        .window("rust-sdl2 demo", grid_mapper.width, grid_mapper.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // Tick CPU
        let raw_instruction = cpu.fetch();
        let instruction = cpu.decode(raw_instruction);
        println!("{:#06X} -> {:?}", raw_instruction, instruction);
        cpu.execute(instruction);

        // Draw VRAM
        for x in 0..grid_mapper.width {
            for y in 0..grid_mapper.height {
                let color = match grid_mapper.get_type(x, y) {
                    GridPixelType::Cell { vram_x, vram_y } => {
                        if cpu.vram().get_cell(vram_x, vram_y) {
                            Color::RGB(255, 255, 255)
                        } else {
                            Color::RGB(0, 0, 0)
                        }
                    }
                    GridPixelType::Divider => Color::RGB(50, 50, 50),
                };
                canvas.set_draw_color(color);
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
