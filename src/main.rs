extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct KeyPresses {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

struct Player {
    rect: Rect,
    color: Color,
}

impl Player {
    fn update(&mut self, key_presses: &KeyPresses) {
        if key_presses.left {
            self.rect.x -= 5;
        }
        if key_presses.right {
            self.rect.x += 5;
        }
        if key_presses.up {
            self.rect.y -= 5;
        }
        if key_presses.down {
            self.rect.y += 5;
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect)
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_rect = Rect::new(0, 0, 800, 600);

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

    let mut player = Player {
        rect: Rect::new(window_rect.w / 2 - 15, window_rect.h / 2 - 15, 30, 30),
        color: Color::RGB(255, 255, 255),
    };

    let mut key_presses = KeyPresses {
        left: false,
        right: false,
        up: false,
        down: false,
    };

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
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => key_presses.left = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => key_presses.left = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => key_presses.right = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => key_presses.right = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => key_presses.up = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => key_presses.up = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => key_presses.down = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => key_presses.down = false,
                _ => {}
            }
        }

        player.update(&key_presses);
        player.render(&mut canvas).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
