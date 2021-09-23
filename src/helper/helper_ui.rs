/// how much of a given size fit in a percentage
///
/// EXAMPLE
/// ```
/// let res = calculate_size_of_percentage(0.5,100); // should be 50
/// assert_eq!(res,50);
/// ```
pub fn calculate_size_of_percentage(percentage_of_size: f32, size_of_element: u32) -> u32 {
  assert!(
    percentage_of_size <= 1.0000001f32 && f32::EPSILON <= percentage_of_size,
    "desired percentage is out of range"
  );
  let result: u32 = (percentage_of_size * size_of_element as f32) as u32;
  result.clamp(1, u32::MAX)
}

/// Does the same as *[calculate_size_of_percentage]* this is just here for code clarity
pub fn calculate_ui_element_width(percentage_of_width: f32, ui_width: u32) -> u32 {
  calculate_size_of_percentage(percentage_of_width, ui_width)
}

/// Does the same as *[calculate_size_of_percentage]* this is just here for code clarity
pub fn calculate_ui_element_height(percentage_of_height: f32, ui_height: u32) -> u32 {
  calculate_size_of_percentage(percentage_of_height, ui_height)
}
