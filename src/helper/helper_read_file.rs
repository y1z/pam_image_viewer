use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const PPM_FORMAT_EXTENTION: &'static str = ".ppm";

pub struct UnsignedNumberParseResult {
  number_parsed: Option<u32>,
  starting_index: usize,
  ending_index: usize,
}

#[derive(Debug)]
pub struct PixelBuffer {
  pub buffer: Vec<Color>,
  pub width: u32,
  pub height: u32,
}

pub fn convert_ppm_file_to_pixel_buffer(path_to_file: String) -> PixelBuffer {
  let file = File::open(&path_to_file); //op(path_to_file);
  let can_open_file = path_to_file.ends_with(PPM_FORMAT_EXTENTION) && file.is_ok();
  let mut buffer: Vec<u8> = vec![];

  let mut result: Vec<Color> = vec![];
  let mut result_width: u32 = 0;
  let mut result_height: u32 = 0;

  if can_open_file {
    let mut reader = BufReader::new(file.unwrap());
    let has_read_successfully = reader.read_to_end(&mut buffer).is_ok();

    if has_read_successfully && buffer[0] == 'P' as u8 && buffer[1] == '6' as u8 {
      let mut buffer_index = 2;

      let width_height_maxval = parse_ppm_width_height_maxval(&buffer, &mut buffer_index);

      println!(
        "width : {}\nheight : {}\nmaxval : {}",
        width_height_maxval[0], width_height_maxval[1], width_height_maxval[2]
      );

      result_width = width_height_maxval[0];
      result_height = width_height_maxval[1];

      result = parse_ppm_RGB_data(
        result_width,
        result_height,
        width_height_maxval[2],
        &mut buffer,
        &mut buffer_index,
      );
    }
  } else {
    panic!(
      "given file [{}] path does not end with the {} extention ",
      path_to_file, PPM_FORMAT_EXTENTION
    );
  }

  PixelBuffer {
    buffer: result,
    width: result_width,
    height: result_height,
  }
}

fn parse_ppm_width_height_maxval(container: &Vec<u8>, index: &mut usize) -> [u32; 3] {
  let mut result = [0u32, 0u32, 0u32];

  for x in 0..result.len() {
    'find_numbers: loop {
      if container[*index].is_ascii_digit() {
        let parsing_result = parse_unsigned_numbers(container, index);
        result[x] = parsing_result.number_parsed.unwrap();
        break 'find_numbers;
      }
      *index += 1;
    }
  }
  // skip empty space
  *index += 1;

  result
}

pub fn parse_unsigned_numbers(container: &Vec<u8>, index: &mut usize) -> UnsignedNumberParseResult {
  let starting_index_ = *index;

  let mut individual_digits: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut digit_index: usize = 0;

  'git_digits: loop {
    let current_value = container[*index];
    let continue_parsing = current_value.is_ascii_digit();
    if !continue_parsing {
      break 'git_digits;
    } else {
      let value_of_digit = current_value - '0' as u8;
      individual_digits[digit_index] = value_of_digit;
      digit_index += 1;
    }

    *index += 1usize;
  }

  let mut resulting_value: u32 = 0;
  let mut mul_digit: u32 = 1;
  for x in (0..digit_index).rev() {
    resulting_value = resulting_value + (individual_digits[x] as u32 * mul_digit);
    mul_digit *= 10;
  }

  UnsignedNumberParseResult {
    number_parsed: Some(resulting_value),
    starting_index: starting_index_,
    ending_index: *index,
  }
}

fn parse_ppm_RGB_data(
  width: u32,
  height: u32,
  max_val: u32,
  container: &Vec<u8>,
  index: &mut usize,
) -> Vec<Color> {
  let size_of_buffer = width as usize * height as usize;
  let mut result: Vec<Color> = Vec::with_capacity(size_of_buffer);
  let starting_value = *index as usize;
  let mut rgb_index: usize = 0;
  let mut current_color: Color = Color::WHITE;
  for x in starting_value..container.len() {
    match rgb_index % 3 {
      0 => current_color.r = container[x],
      1 => current_color.g = container[x],
      2 => {
        current_color.b = container[x];
        result.push(current_color);
      }
      _ => {}
    }

    *index += 1;
    rgb_index += 1;
  }

  result
}
