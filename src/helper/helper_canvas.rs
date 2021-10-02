use crate::helper::helper_render_mode;
use sdl2::pixels::Color;

pub fn help_get_canvas_display_mode(canvas: &sdl2::render::WindowCanvas) -> (i32, i32) {
  let display_mode_some = canvas.window().display_mode();

  return match display_mode_some.is_ok() {
    true => {
      let dm = display_mode_some.unwrap();
      (dm.w, dm.h)
    }
    false => (-1, -1),
  };
}

pub fn help_render_pixel_buffer(
  canvas: &mut sdl2::render::WindowCanvas,
  pixel_buffer: &[Color],
  top_left_position: (i32, i32),
  width_and_height_pixel_buffer: (u32, u32),
  scale_of_pixel: Option<(u32, u32)>,
) {
  let final_scale_of_pixel = match scale_of_pixel {
    Some(x) => {
      assert_ne!(x.0, 0);
      assert_ne!(x.1, 0);
      x
    }
    _ => (1u32, 1u32),
  };

  let mut rect = sdl2::rect::Rect::new(
    top_left_position.0,
    top_left_position.1,
    final_scale_of_pixel.0,
    final_scale_of_pixel.1,
  );
  let initial_rect_position = (rect.x(), rect.y());
  let width = width_and_height_pixel_buffer.0;
  let height = width_and_height_pixel_buffer.1;
  for y in 0usize..height as usize {
    for x in 0usize..width as usize {
      let color_index = (width as usize * y) + x;
      rect.set_x(initial_rect_position.0 + (final_scale_of_pixel.0 * x as u32) as i32);
      rect.set_y(initial_rect_position.1 + (final_scale_of_pixel.1 * y as u32) as i32);

      canvas.set_draw_color(pixel_buffer[color_index]);
      canvas.fill_rect(rect);
    }
  }
}

pub fn help_render_pixel_buffer_in_area(
  area: (u32, u32),
  canvas: &mut sdl2::render::WindowCanvas,
  pixel_buffer: &[Color],
  top_left_position: (i32, i32),
  width_and_height_pixel_buffer: (u32, u32),
) {
  assert_ne!(area.0, 0);
  assert_ne!(area.1, 0);
  assert_ne!(width_and_height_pixel_buffer.0, 0);
  assert_ne!(width_and_height_pixel_buffer.1, 0);

  let scale_x = {
    let mut result = area.0 / width_and_height_pixel_buffer.0;
    if result == 0 {
      result = 1;
    }
    result
  };
  let scale_y = {
    let mut result = area.1 / width_and_height_pixel_buffer.1;
    if result == 0 {
      result = 1;
    }
    result
  };

  let scale = Some((scale_x, scale_y));

  help_render_pixel_buffer(
    canvas,
    pixel_buffer,
    top_left_position,
    width_and_height_pixel_buffer,
    scale,
  );
}
