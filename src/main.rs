//extern crate sdl2;
pub mod helper {
  pub mod helper_canvas;
}

use helper::helper_canvas::help_get_canvas_display_mode;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

pub fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("rust-sdl2 demo", DEFAULT_WIDTH, DEFAULT_HEIGHT)
    .position_centered()
    .resizable()
    .build()
    .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();
  canvas.set_logical_size(DEFAULT_WIDTH, DEFAULT_HEIGHT);
  canvas.set_draw_color(sdl2::pixels::Color::BLACK);
  canvas.clear();
  canvas.present();
  let mut current_rect = Rect::new(0, 0, 100 / 4, 100 / 4);
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
    i = (i + 1) % 255;
    let canvas_size = canvas.logical_size();
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    canvas.fill_rect(current_rect);
    let current_x = current_rect.x();
    current_rect.set_x((current_x + 1) % canvas_size.0 as i32);
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { timestamp: u32 } => break 'running,
        Event::KeyDown {
          timestamp,
          window_id,
          keycode,
          scancode,
          keymod,
          repeat,
          ..
        } => {
          let raw_keycode = keycode.unwrap();
          match raw_keycode {
            Keycode::W => {
              println!("you are pressing the {} key", raw_keycode);
            }
            _ => {}
          }
        }
        Event::Window { win_event, .. } => match win_event {
          sdl2::event::WindowEvent::Resized(x, y) => {
            let canvas_display_mode = help_get_canvas_display_mode(&canvas);
            println!(
              "\n display_mode width [{}] and height [{}]\nResized width [{}] height [{}]",
              canvas_display_mode.0, canvas_display_mode.1, x, y
            );
          }
          sdl2::event::WindowEvent::SizeChanged(x, y) => {
            canvas.set_logical_size(x as u32, y as u32);
            let size = canvas.logical_size();
            println!("logical size width [{}] height [{}]", size.0, size.1);
          }
          _ => {}
        },
        _ => {}
      }
    }
    // The rest of the game loop goes here...
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
