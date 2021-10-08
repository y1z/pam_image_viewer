use crate::helper_consts::*;
use crate::helper_parse_functions::*;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn convert_ppm_file_to_pixel_buffer(path_to_file: String) -> Option<PixelBuffer> {
  let file = File::open(&path_to_file); //op(path_to_file);
  let can_open_file = path_to_file.ends_with(PPM_FORMAT_EXTENTION) && file.is_ok();
  let mut buffer: Vec<u8> = vec![];

  let mut result: Vec<Color> = vec![];
  let mut result_width: u32 = 0;
  let mut result_height: u32 = 0;

  let mut result: PixelBuffer;

  if can_open_file {
    let mut reader = BufReader::new(file.unwrap());
    let has_read_successfully = reader.read_to_end(&mut buffer).is_ok();

    if has_read_successfully && is_valid_ppm_header(&buffer) {
      return Some(parse_ppm_data(&buffer));
    }
  } else {
    panic!(
      "given file [{}] path does not end with the {} extention ",
      path_to_file, PPM_FORMAT_EXTENTION
    );
  }

  None
}

pub fn convert_pam_file_to_pixel_buffer(path_to_file: String) -> Option<PixelBuffer> {
  let file = File::open(&path_to_file);
  let can_open_file = path_to_file.ends_with(PAM_FORMAT_EXTENTION) && file.is_ok();
  let mut buffer: Vec<u8> = vec![];

  let mut result: Vec<Color> = vec![];
  let mut result_width: u32 = 0;
  let mut result_height: u32 = 0;

  if can_open_file {
    let mut reader = BufReader::new(file.unwrap());
    let mut buffer: String = String::new();
    let reader_result = reader.read_to_string(&mut buffer);
    if reader_result.is_err() {
      panic!(
        "Error from reading [{}] : {} ",
        path_to_file,
        reader_result.unwrap_err()
      );
    }

    let pam_header = parse_pam_header(&path_to_file);
  } else {
    eprintln!("cannot open file {}", path_to_file);
    return None;
  }

  Some(PixelBuffer {
    buffer: result,
    width: result_width,
    height: result_height,
  })
}
