extern crate sdl2;

use Vec2::Vec2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect};

use std::time::Duration;



pub fn render(canvas: &mut WindowCanvas){
    canvas.fill_rect(Rect::new(10, 10, 10, 10));
}

pub fn main() {
    let mut a: Vec2;
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();

    let window = video_subsystem.window("something", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = _sdl.event_pump().unwrap();
    let mut i = 0;

    

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255,255,255));
        render(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}