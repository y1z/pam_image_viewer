//extern crate sdl2;
pub mod helper {
  pub mod helper_canvas;
  pub mod helper_ui;
}

use helper::helper_canvas::help_get_canvas_display_mode;
use helper::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

pub struct BasicSdlSystem {
  pub sdl_context: sdl2::Sdl,
  pub video_subsystem: sdl2::VideoSubsystem,
}

type MainReturn = std::result::Result<(), String>;

pub fn main() -> MainReturn {
  run()
}

pub fn init_basic_sdl_system() -> BasicSdlSystem {
  let sdl_context_ = sdl2::init();
  if sdl_context_.is_err() {
    panic!("could not start SDL2 (could not start the context)");
  }
  let raw_sdl_context_ = sdl_context_.unwrap();
  let v_subsystem = raw_sdl_context_.video();
  if v_subsystem.is_err() {
    panic!("could not start the SDL2 video subsystem");
  }

  BasicSdlSystem {
    sdl_context: raw_sdl_context_,
    video_subsystem: v_subsystem.unwrap(),
  }
}

pub fn init_window(
  video_subsystem: &mut sdl2::VideoSubsystem,
  width: u32,
  height: u32,
) -> sdl2::video::Window {
  video_subsystem
    .window("PIV", width, height)
    .position_centered()
    .resizable()
    .build()
    .unwrap()
}

pub fn init_canvas(
  window: sdl2::video::Window,
  width: u32,
  height: u32,
) -> sdl2::render::WindowCanvas {
  let mut res = window.into_canvas().build().unwrap();
  res.set_logical_size(width, height);
  return res;
}

pub fn run() -> MainReturn {
  let mut basic_sdl_system = init_basic_sdl_system();
  let mut window = init_window(
    &mut basic_sdl_system.video_subsystem,
    DEFAULT_WIDTH,
    DEFAULT_HEIGHT,
  );
  let mut canvas = init_canvas(window, DEFAULT_WIDTH, DEFAULT_HEIGHT);

  return main_loop(basic_sdl_system, &mut canvas);
}

pub fn main_loop(
  sdl_system: BasicSdlSystem,
  canvas: &mut sdl2::render::WindowCanvas,
) -> MainReturn {
  let mut current_rect = Rect::new(0, 0, 10, 10);
  let mut event_pump = sdl_system.sdl_context.event_pump().unwrap();
  let mut delta_time = std::time::Duration::new(0, 0);
  let mut seconds_passed = delta_time.as_secs_f32();

  let pixel_buffer = [
    Color::BLUE,
    Color::RED,
    Color::BLUE,
    Color::RED,
    Color::BLUE,
    Color::RED,
    Color::BLUE,
    Color::RED,
    Color::BLUE,
    Color::RED,
    Color::GREEN,
    Color::BLACK,
    Color::GREEN,
    Color::BLACK,
    Color::GREEN,
    Color::BLACK,
    Color::GREEN,
    Color::BLACK,
    Color::GREEN,
    Color::BLACK,
    Color::WHITE,
    Color::WHITE,
    Color::WHITE,
    Color::WHITE,
    Color::WHITE,
  ];
  let pixel_buffer_width = 5u32;
  let pixel_buffer_height = 5u32;

  'running: loop {
    let start_time = std::time::SystemTime::now();
    seconds_passed += delta_time.as_secs_f32();

    let sin_wave_abs = seconds_passed.sin().abs();
    let cos_wave_abs = seconds_passed.cos().abs();
    let sin_wave = seconds_passed.sin();
    let canvas_size = canvas.logical_size();
    // #B524BE purple
    canvas.set_draw_color(Color::RGB(0xB5, 0x24, 0xBE));
    canvas.clear();

    helper_canvas::help_render_pixel_buffer_in_area(
      (
        helper_ui::calculate_ui_element_width(sin_wave_abs.clamp(0.01, 1.0), canvas_size.0),
        100u32,
      ),
      canvas,
      &pixel_buffer,
      (0, 0),
      (5, 5),
    );

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
        } => {}
        Event::Window { win_event, .. } => match win_event {
          sdl2::event::WindowEvent::Resized(x, y) => {
            let canvas_display_mode = help_get_canvas_display_mode(&canvas);
            println!(
              "\nDisplay_mode width [{}] and height [{}]\nResized width [{}] height [{}]",
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
    let end_timer = std::time::SystemTime::now();
    delta_time = end_timer.duration_since(start_time).unwrap()
  }
  return Ok(());
}
