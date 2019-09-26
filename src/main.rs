extern crate backtrace;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::panic;

#[macro_use]
mod extra;
#[macro_use]
mod game;
mod build;

fn main() {
    // handle panics
    panic::set_hook(Box::new(extra::panic_handler));

    // game title
    const GAME_TITLE: &'static str = "3 in row";
    // game window size
    const W_WIDTH: u32 = 640;
    const W_HEIGHT: u32 = 640;
    const MILLISECOND: u32 = 1000;
    
    // default colors
    let bg_color = Color::RGB(100, 100, 100);

    // SDL2
    let sdl_context = sdl2::init().expect("Can't init sdl2 context");
    let video_subsystem = sdl_context.video().expect("Can't create video subsystem");
    let window =
        video_subsystem.window(GAME_TITLE, W_WIDTH, W_HEIGHT).position_centered().build().expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't get canvas");
    let mut timer = msg!(sdl_context.timer(); canvas.window(), GAME_TITLE);

    // fps block
    let fps = 60;
    let mut last_time = timer.ticks();
    let mut field = game::Field::new();
    field.set_tile_size(point!(64));
    field.set_field_size(point!(10));

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(bg_color);
        canvas.clear();
        field.render(&mut canvas);
        canvas.present();

        // events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // fps counter
        let current_time = timer.ticks();
        let elapsed = current_time - last_time;
        last_time = current_time;

        // sleep
        let sleep_time = if elapsed < MILLISECOND / fps { MILLISECOND / fps - elapsed } else { MILLISECOND / fps };
        if sleep_time > 0 {
            timer.delay(sleep_time);
        }
    }
}
