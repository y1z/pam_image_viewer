use std::ops::{BitAnd, BitOr};

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderModeBitMasks {
  UNKNOWN = 0,
  RED_ONLY = (1 << 0),
  GREEN_ONLY = (1 << 1),
  BLUE_ONLY = (1 << 2),
  ALPHA_ONLY = (1 << 3),
  RGB = RenderModeBitMasks::RED_ONLY as u32
    | RenderModeBitMasks::GREEN_ONLY as u32
    | RenderModeBitMasks::BLUE_ONLY as u32,
  RGBA = RenderModeBitMasks::RGB as u32 | RenderModeBitMasks::ALPHA_ONLY as u32,
  GRAY_SCALE = (1 << 8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderMode {
  pub bit_mask: u32,
}

impl RenderMode {
  const RGB_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::RGB as u32,
  };

  pub fn create(input_bit_mask: RenderModeBitMasks) -> RenderMode {
    RenderMode::from(input_bit_mask as u32)
  }

  pub fn new() -> RenderMode {
    RenderMode::from(RenderModeBitMasks::UNKNOWN as u32)
  }

  pub fn has_bitmask(&self, input_bit_mask: RenderModeBitMasks) -> bool {
    (self.bit_mask & input_bit_mask as u32) > 0
  }
}

impl From<u32> for RenderMode {
  fn from(input_value: u32) -> Self {
    let is_valid_bitmask = (input_value & RenderModeBitMasks::RGBA as u32) > 0
      || (input_value & RenderModeBitMasks::GRAY_SCALE as u32) > 0;

    if is_valid_bitmask {
      return RenderMode {
        bit_mask: input_value,
      };
    }

    return RenderMode {
      bit_mask: RenderModeBitMasks::UNKNOWN as u32,
    };
  }
}

impl BitAnd for RenderModeBitMasks {
  type Output = bool;
  fn bitand(self, rhs: Self) -> Self::Output {
    (self as u32 & rhs as u32) > 0
  }
}
