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
  width_and_height: (u32, u32),
  scale_of_pixel: Option<u32>,
) {
  let final_scale_of_pixel = match scale_of_pixel {
    Some(x) => {
      assert_ne!(x, 0);
      x
    }
    _ => 1u32,
  };

  let mut rect = sdl2::rect::Rect::new(
    top_left_position.0,
    top_left_position.1,
    final_scale_of_pixel,
    final_scale_of_pixel,
  );
  let initial_rect_position = (rect.x(), rect.y());

  for y in 0usize..width_and_height.1 as usize {
    for x in 0usize..width_and_height.0 as usize {
      let color_index = (width_and_height.0 as usize * y) + x;
      rect.set_x(initial_rect_position.0 + (final_scale_of_pixel * x as u32) as i32);
      rect.set_y(initial_rect_position.1 + (final_scale_of_pixel * y as u32) as i32);

      canvas.set_draw_color(pixel_buffer[color_index]);
      canvas.fill_rect(rect);
    }
  }
}
