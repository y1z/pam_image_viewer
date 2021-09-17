use sdl2::render::Canvas;

pub fn help_canvas_size(canvas: &sdl2::render::WindowCanvas) -> (i32, i32) {
  let display_mode_some = canvas.window().display_mode();

  return match display_mode_some.is_ok() {
    true => {
      let dm = display_mode_some.unwrap();
      (dm.w, dm.h)
    }
    false => (-1, -1),
  };
}
