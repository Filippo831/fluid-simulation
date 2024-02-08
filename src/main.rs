extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

mod fluid_cube;
use fluid_cube::Fluid;

const ITER: u32 = 10;

const SIZE: u32= 128;

const GRID_X_SIZE: u32 = SIZE;
const GRID_Y_SIZE: u32 = SIZE;
const DOT_SIZE_IN_PXS: u32 = 15;

fn render(canvas: &mut WindowCanvas, i: u32) -> Result<(), String> {
    for a in 0..GRID_X_SIZE {
        for b in 0..GRID_Y_SIZE {
            canvas.set_draw_color(Color::RGB(((i + a) * 5 % 255) as u8, ((i + b) * 5 % 255) as u8, 50));
            let current_frame = Rect::new(
                (a * DOT_SIZE_IN_PXS) as i32,
                (b * DOT_SIZE_IN_PXS) as i32,
                DOT_SIZE_IN_PXS,
                DOT_SIZE_IN_PXS,
            );
            canvas.fill_rect(current_frame).unwrap();
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake Game",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i :u32= 0;

    let fluid = Fluid::new(SIZE, 0.1, 0.0, 0.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        i = i + 1;
        canvas.clear();
        render(&mut canvas, i)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
