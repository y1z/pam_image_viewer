use sdl2::pixels::Color;
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderMode {
  pub bit_mask: u32,
}

impl RenderMode {
  pub const RGB_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::RGB as u32,
  };

  pub const RGBA_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::RGBA as u32,
  };

  pub const RED_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::RED_ONLY as u32,
  };

  pub const GREEN_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::GREEN_ONLY as u32,
  };

  pub const BLUE_RENDER_MODE: RenderMode = RenderMode {
    bit_mask: RenderModeBitMasks::BLUE_ONLY as u32,
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

pub fn filter_color(original_color: &Color, render_mode: &RenderMode) -> Color {
  if RenderMode::RGB_RENDER_MODE == *render_mode {
    return Color::from((original_color.r, original_color.g, original_color.b));
  } else if RenderMode::RGBA_RENDER_MODE == *render_mode {
    return Color::from((
      original_color.r,
      original_color.g,
      original_color.b,
      original_color.a,
    ));
  }

  let mut result: Color = Color::from((0, 0, 0));
  if render_mode.has_bitmask(RenderModeBitMasks::RED_ONLY) {
    result.r = original_color.r
  }

  if render_mode.has_bitmask(RenderModeBitMasks::GREEN_ONLY) {
    result.g = original_color.g
  }

  if render_mode.has_bitmask(RenderModeBitMasks::BLUE_ONLY) {
    result.b = original_color.b
  }

  if render_mode.has_bitmask(RenderModeBitMasks::ALPHA_ONLY) {
    result.a = original_color.a
  }

  result
}
