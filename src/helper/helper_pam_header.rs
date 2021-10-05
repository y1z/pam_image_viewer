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

impl std::fmt::Display for TupleTypes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Used to know how the data of the .pam file is organized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PamHeader {
  height: u32,
  width: u32,
  depth: u32,
  max_val: u16,
  tuple_types: TupleTypes,
}
