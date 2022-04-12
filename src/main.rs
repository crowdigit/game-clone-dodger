extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    struct Rect {
        w: i16,
        h: i16,
    }

    struct Point {
        x: i16,
        y: i16,
    }

    let window_rect = Rect { w: 800, h: 600 };

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            window_rect.w.try_into().unwrap(),
            window_rect.h.try_into().unwrap(),
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
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

        let white = Color::RGB(255, 255, 255);
        canvas
            .filled_circle(window_rect.w / 2, window_rect.h / 2, 64, white)
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
