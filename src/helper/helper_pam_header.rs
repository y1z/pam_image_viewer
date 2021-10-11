/**
*  Example header from : http://netpbm.sourceforge.net/doc/pam.html#layout
P7
WIDTH 227
HEIGHT 149
DEPTH 3
MAXVAL 255
TUPLTYPE RGB
ENDHDR
*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseOrFindMethod {
  FIND_START = 0,
  PARSE_WIDTH,
  PARSE_HEIGHT,
  PARSE_DEPTH,
  PARSE_MAXVAL,
  PARSE_TUPLTYPE,
  FIND_END,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringAndMethod {
  pub string: &'static str,
  pub parse_method: ParseOrFindMethod,
}

impl StringAndMethod {
  pub const fn from(
    string_input: &'static str,
    parse_method_input: ParseOrFindMethod,
  ) -> StringAndMethod {
    StringAndMethod {
      string: string_input,
      parse_method: parse_method_input,
    }
  }
}

pub const PAM_HEADER_EXPECTED_STRS_METHODS: [StringAndMethod; 7] = [
  StringAndMethod::from("P7", ParseOrFindMethod::FIND_START),
  StringAndMethod::from("WIDTH", ParseOrFindMethod::PARSE_WIDTH),
  StringAndMethod::from("HEIGHT", ParseOrFindMethod::PARSE_HEIGHT),
  StringAndMethod::from("DEPTH", ParseOrFindMethod::PARSE_DEPTH),
  StringAndMethod::from("MAXVAL", ParseOrFindMethod::PARSE_MAXVAL),
  StringAndMethod::from("TUPLTYPE", ParseOrFindMethod::PARSE_TUPLTYPE),
  StringAndMethod::from("ENDHDR", ParseOrFindMethod::FIND_END),
];

/// enum are based on this http://netpbm.sourceforge.net/doc/pam.html#tupletype
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TupleTypes {
  UNDEFINED = 0,
  BLACKANDWHITE = 1,
  GRAYSCALE = 2,
  RGB = 4,
  BLACKANDWHITE_ALPHA = TupleTypes::BLACKANDWHITE as u32 | TupleTypes::ALPHA as u32,
  GRAYSCALE_ALPHA = TupleTypes::GRAYSCALE as u32 | TupleTypes::ALPHA as u32,
  RGB_ALPHA = TupleTypes::RGB as u32 | TupleTypes::ALPHA as u32,
  ALPHA = (1 << 31),
}

impl TupleTypes {
  pub const fn get_value(&self) -> u32 {
    *self as u32
  }

  pub fn iterator() -> impl Iterator<Item = TupleTypes> {
    [
      TupleTypes::UNDEFINED,
      TupleTypes::BLACKANDWHITE,
      TupleTypes::GRAYSCALE,
      TupleTypes::RGB,
      TupleTypes::BLACKANDWHITE_ALPHA,
      TupleTypes::GRAYSCALE_ALPHA,
      TupleTypes::GRAYSCALE_ALPHA,
      TupleTypes::RGB_ALPHA,
    ]
    .iter()
    .copied()
  }
}

impl std::fmt::Display for TupleTypes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Used to know how the data of the .pam file is organized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PamHeader {
  pub height: u32,
  pub width: u32,
  pub depth: u32,
  pub max_val: u16,
  pub tuple_types: TupleTypes,
}
