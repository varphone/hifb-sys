#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const FBIOGET_COLORKEY_HIFB: u32 = 2148025946;
pub const FBIOPUT_COLORKEY_HIFB: u32 = 1074284123;
pub const FBIOGET_ALPHA_HIFB: u32 = 2148288092;
pub const FBIOPUT_ALPHA_HIFB: u32 = 1074546269;
pub const FBIOGET_SCREEN_ORIGIN_HIFB: u32 = 2148025950;
pub const FBIOPUT_SCREEN_ORIGIN_HIFB: u32 = 1074284127;
pub const FBIOGET_DEFLICKER_HIFB: u32 = 2149074530;
pub const FBIOPUT_DEFLICKER_HIFB: u32 = 1075332707;
pub const FBIOGET_VBLANK_HIFB: u32 = 18020;
pub const FBIOPUT_SHOW_HIFB: u32 = 1074021989;
pub const FBIOGET_SHOW_HIFB: u32 = 2147763814;
pub const FBIOGET_CAPABILITY_HIFB: u32 = 2160608871;
pub const FBIOPUT_SCREENSIZE: u32 = 1074284162;
pub const FBIOGET_SCREENSIZE: u32 = 2148025987;
pub const FBIOFLIP_SURFACE: u32 = 1075857028;
pub const FBIOPUT_COMPRESSION_HIFB: u32 = 1074022021;
pub const FBIOGET_COMPRESSION_HIFB: u32 = 2147763846;
pub const FBIOPUT_DYNAMIC_RANGE_HIFB: u32 = 1074022027;
pub const FBIOGET_DYNAMIC_RANGE_HIFB: u32 = 2147763852;
pub const FBIO_RELEASE_HIFB: u32 = 18070;

impl fb_bitfield {
    /// Create a new fb_bitfield.
    pub fn new(offset: u32, length: u32, msb_right: u32) -> Self {
        Self {
            offset,
            length,
            msb_right,
        }
    }
}

/// Extract the color channel in bitfield.
pub fn hifb_rgb(bitfield: &fb_bitfield, color: u32) -> u8 {
    let h = ((color >> bitfield.offset) << (8 - bitfield.length)) & 0xffu32;
    let l = (h >> bitfield.length) & 0xffu32;
    (h + l) as u8
}

/// Convert color to key color.
pub fn hifb_color2key(var_info: &fb_var_screeninfo, color: u32) -> u32 {
    match var_info.bits_per_pixel {
        0..=8 => color,
        _ => {
            let r = hifb_rgb(&var_info.red, color) as u32;
            let g = hifb_rgb(&var_info.green, color) as u32;
            let b = hifb_rgb(&var_info.blue, color) as u32;
            (r << 16) + (g << 8) + b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hifb_rgb() {
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 5, 0), 0xffff_ffffu32), 0xffu8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 5, 0), 0xff7f_7f7fu32), 0xffu8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 5, 0), 0xff55_5555u32), 0xADu8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 5, 0), 0x0000_0000u32), 0x00u8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 8, 0), 0xffff_ffffu32), 0xffu8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 8, 0), 0xff7f_7f7fu32), 0x7fu8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 8, 0), 0xff55_5555u32), 0x55u8);
        assert_eq!(hifb_rgb(&fb_bitfield::new(0, 8, 0), 0x0000_0000u32), 0x00u8);
    }

    #[test]
    fn test_hifb_color2key() {
        let mut v1: fb_var_screeninfo = Default::default();
        v1.bits_per_pixel = 24;
        v1.red = fb_bitfield::new(0, 8, 0);
        v1.green = fb_bitfield::new(8, 8, 0);
        v1.blue = fb_bitfield::new(16, 8, 0);
        v1.transp = fb_bitfield::new(0, 0, 0);
        assert_eq!(hifb_color2key(&v1, 0xffff_ffffu32), 0x00ff_ffffu32);
        assert_eq!(hifb_color2key(&v1, 0xff7f_7f7fu32), 0x007f_7f7fu32);
        assert_eq!(hifb_color2key(&v1, 0xff55_5555u32), 0x0055_5555u32);
        assert_eq!(hifb_color2key(&v1, 0x0000_0000u32), 0x0000_0000u32);
    }
}
