use sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use chip_8::CHIP8_WIDTH;
use chip_8::CHIP8_HEIGHT;

const SCALE: u32 = 16;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            canvas,
        }
    }

    pub fn draw(&mut self, screen: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        println!("help");
        for (y, row) in screen.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }

        self.canvas.present();
    }
}

fn color(value: u8) -> Color {
    if value == 0 {
        Color::RGB(0, 0, 0)
    } else {
        Color::RGB(0, 250, 0)
    }
}
