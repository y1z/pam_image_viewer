use crate::helper_consts::*;
use crate::helper_pam_header::*;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct UnsignedNumberParseResult {
  number_parsed: Option<u32>,
  starting_index: usize,
  ending_index: usize,
}
//
#[derive(Debug)]
pub struct PixelBuffer {
  pub buffer: Vec<Color>,
  pub width: u32,
  pub height: u32,
}

pub fn parse_ppm_width_height_maxval(container: &Vec<u8>, index: &mut usize) -> [u32; 3] {
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
  //*index += 1;

  result
}

pub fn is_valid_ppm_header(container: &[u8]) -> bool {
  let has_magic_number = container[0] == 'P' as u8 && container[1] == '6' as u8;
  has_magic_number
}

// &[u8]
pub fn parse_unsigned_numbers(container: &[u8], index: &mut usize) -> UnsignedNumberParseResult {
  let starting_index_ = *index;

  let mut individual_digits: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut digit_index: usize = 0;
  let container_len = container.len();
  let mut continue_parsing = true;

  while continue_parsing {
    let is_in_range = *index < container_len;

    if is_in_range {
      continue_parsing = container[*index].is_ascii_digit();

      if continue_parsing {
        let current_value = container[*index];
        let value_of_digit = current_value - '0' as u8;
        individual_digits[digit_index] = value_of_digit;
        digit_index += 1;
      }
    } else {
      continue_parsing = false;
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

pub fn parse_unsigned_numbers_string(
  container: &String,
  index: &mut usize,
) -> UnsignedNumberParseResult {
  parse_unsigned_numbers(container.as_bytes(), index)
}

pub fn parse_unsigned_numbers_vec_u8(
  container: &Vec<u8>,
  index: &mut usize,
) -> UnsignedNumberParseResult {
  return parse_unsigned_numbers(container, index);
}

pub fn parse_ppm_data(buffer: &Vec<u8>) -> PixelBuffer {
  let mut buffer_index = 2;
  let width_height_maxval = parse_ppm_width_height_maxval(buffer, &mut buffer_index);

  println!(
    "width : {}\nheight : {}\nmaxval : {}",
    width_height_maxval[0], width_height_maxval[1], width_height_maxval[2]
  );

  let result_width = width_height_maxval[0];
  let result_height = width_height_maxval[1];

  let result = parse_ppm_RGB_data(
    result_width,
    result_height,
    width_height_maxval[2],
    &buffer,
    &mut buffer_index,
  );

  PixelBuffer {
    buffer: result,
    width: result_width,
    height: result_height,
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

/// extracts the data of the pam header : http://netpbm.sourceforge.net/doc/pam.html#layout
/// It's assumed to be at the start of the .pam file
pub fn parse_pam_header(buffer: &mut BufReader<File>) -> Option<PamHeader> {
  // string and the mothod needed to parse them
  let strs_methods = &PAM_HEADER_EXPECTED_STRS_METHODS;
  // keeps track of which string was already parsed
  let mut found_and_parsed_strings: [(bool, crate::helper_pam_header::StringAndMethod); 7] = [
    (false, strs_methods[0]),
    (false, strs_methods[1]),
    (false, strs_methods[2]),
    (false, strs_methods[3]),
    (false, strs_methods[4]),
    (false, strs_methods[5]),
    (false, strs_methods[6]),
  ];

  let get_value_from_line = |line: &str| -> u32 {
    let mut res = 1u32;
    if let Some(digit_index) = line.find(|c: char| c.is_ascii_digit()) {
      let mut mut_digit_index = digit_index;
      let result = parse_unsigned_numbers(line.as_bytes(), &mut mut_digit_index);
      res = result.number_parsed.unwrap();
    }
    res
  };

  let mut width_res: u32 = 1;
  let mut height_res: u32 = 1;
  let mut depth_res: u32 = 1;
  let mut max_val_res: u16 = 1;
  let mut tuple_type_res: TupleTypes = TupleTypes::UNDEFINED;
  let mut found_count: usize = 0;
  let mut past_end_header_index_res: usize = 0;

  'get_every_line: loop {
    if !(found_count < strs_methods.len()) {
      break 'get_every_line;
    }

    let mut string_line = String::new();
    let line_count = buffer.read_line(&mut string_line);
    let string_clone = String::from(string_line.as_str());
    let raw_line_count = match line_count {
      Ok(x) => x,
      Err(x) => panic!("\n-------------------\n The error is [{}]", x),
    };

    past_end_header_index_res = past_end_header_index_res + raw_line_count;
    for string in found_and_parsed_strings.iter_mut() {
      let find_string = string_clone.find(string.1.string);

      if !string.0 && find_string.is_some() {
        string.0 = true;
        found_count = found_count + 1;
        let string_and_method = string.1;

        match string_and_method.parse_method {
          ParseOrFindMethod::FIND_START | ParseOrFindMethod::FIND_END => {
            break;
          }
          ParseOrFindMethod::PARSE_WIDTH => {
            width_res = get_value_from_line(&string_line);
            break;
          }
          ParseOrFindMethod::PARSE_DEPTH => {
            depth_res = get_value_from_line(&string_line);
            break;
          }
          ParseOrFindMethod::PARSE_HEIGHT => {
            height_res = get_value_from_line(&string_line);
            break;
          }
          ParseOrFindMethod::PARSE_MAXVAL => {
            let temp = get_value_from_line(&string_line);
            if temp > (u16::MAX - 1) as u32 {
              panic!(
                "Value of MAXVAL cannot be bigger than {} and it's {} ",
                (u16::MAX - 1),
                temp
              );
            }
            max_val_res = temp as u16;
            break;
          }
          ParseOrFindMethod::PARSE_TUPLTYPE => {
            tuple_type_res = parse_tuple_type(&String::from(string_clone.as_str()));
            break;
          }
        }
      }
    }
  }

  /// check that each of the necesary strings where found
  for index in 0..found_and_parsed_strings.len() {
    if !found_and_parsed_strings[index].0 {
      eprintln!(
        "Pam image missing the [{}] in the header",
        found_and_parsed_strings[index].1.string
      );
      return None;
    }
  }

  Some(PamHeader {
    past_end_header_index: past_end_header_index_res,
    height: height_res,
    width: width_res,
    depth: depth_res,
    max_val: max_val_res,
    tuple_types: tuple_type_res,
  })
}

fn parse_tuple_type(container: &String) -> TupleTypes {
  let space_separated_words: Vec<_> = container
    .split(|c: char| c == '\n' || c == ' ')
    .filter(|x| *x != "TUPLTYPE")
    .collect();
  let mut result = TupleTypes::UNDEFINED;

  'check_word: for word in space_separated_words {
    for tuple_type in TupleTypes::iterator() {
      if tuple_type.to_string() == word {
        result = tuple_type;
        break 'check_word;
      }
    }
  }

  result
}

pub fn parse_pam_data(
  pam_header: PamHeader,
  pam_data: &mut BufReader<File>,
) -> Option<PixelBuffer> {
  let tuple_type_string = pam_header.tuple_types.to_string();
  pam_data.seek(std::io::SeekFrom::Start(0));
  /**
  UNDEFINED = 0,
  BLACKANDWHITE = 1,
  GRAYSCALE = 2,
  RGB = 4,
  BLACKANDWHITE_ALPHA = TupleTypes::BLACKANDWHITE as u32 | TupleTypes::ALPHA as u32,
  GRAYSCALE_ALPHA = TupleTypes::GRAYSCALE as u32 | TupleTypes::ALPHA as u32,
  RGB_ALPHA = TupleTypes::RGB as u32 | TupleTypes::ALPHA as u32,
  ALPHA = (1 << 31),
   *
  */
  let result = match tuple_type_string.as_str() {
    "RGB" => Some(parse_RGB_pam_data(pam_header, pam_data)),

    _ => None,
  };

  result
}

// TODO : update this to work with 16-bit values
fn parse_RGB_pam_data(pam_header: PamHeader, pam_data: &mut BufReader<File>) -> PixelBuffer {
  let mut buffer: Vec<u8> = vec![];
  if let Err(error_type) = pam_data.read_to_end(&mut buffer) {
    panic!("\n------------------\n the error is [{}]", error_type);
  }
  let capacity = (pam_header.width * pam_header.height) as usize;
  let mut result: Vec<Color> = Vec::<Color>::with_capacity(capacity);
  result.resize_with(capacity, || Color::from((0u8, 0u8, 0u8)));
  // Note : remove variable past_end_header_index because for some reason
  let starting_index = pam_header.past_end_header_index;

  for z in 0..pam_header.depth {
    for y in 0..pam_header.height {
      for x in 0..pam_header.width {
        let current_row = (y * pam_header.width) as usize;
        let current_index = starting_index
          + (z as usize * (pam_header.width * pam_header.height) as usize)
          + current_row as usize
          + x as usize;
        match z {
          0 => result[current_row + x as usize].r = buffer[current_index],
          1 => result[current_row + x as usize].g = buffer[current_index],
          2 => result[current_row + x as usize].b = buffer[current_index],
          3 => result[current_row + x as usize].a = buffer[current_index],
          _ => {}
        }
      }
    }
  }

  PixelBuffer {
    buffer: result,
    width: pam_header.width,
    height: pam_header.height,
  }
}
