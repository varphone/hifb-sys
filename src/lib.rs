#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::op_ref)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const FBIOGET_COLORKEY_HIFB: u32 = 2_148_025_946;
pub const FBIOPUT_COLORKEY_HIFB: u32 = 1_074_284_123;
pub const FBIOGET_ALPHA_HIFB: u32 = 2_148_288_092;
pub const FBIOPUT_ALPHA_HIFB: u32 = 1_074_546_269;
pub const FBIOGET_SCREEN_ORIGIN_HIFB: u32 = 2_148_025_950;
pub const FBIOPUT_SCREEN_ORIGIN_HIFB: u32 = 1_074_284_127;
pub const FBIOGET_DEFLICKER_HIFB: u32 = 2_149_074_530;
pub const FBIOPUT_DEFLICKER_HIFB: u32 = 1_075_332_707;
pub const FBIOGET_VBLANK_HIFB: u32 = 18_020;
pub const FBIOPUT_SHOW_HIFB: u32 = 1_074_021_989;
pub const FBIOGET_SHOW_HIFB: u32 = 2_147_763_814;
pub const FBIOGET_CAPABILITY_HIFB: u32 = 2_160_608_871;
pub const FBIOPUT_SCREENSIZE: u32 = 1_074_284_162;
pub const FBIOGET_SCREENSIZE: u32 = 2_148_025_987;
pub const FBIOFLIP_SURFACE: u32 = 1_075_857_028;
pub const FBIOPUT_COMPRESSION_HIFB: u32 = 1_074_022_021;
pub const FBIOGET_COMPRESSION_HIFB: u32 = 2_147_763_846;
pub const FBIOPUT_DYNAMIC_RANGE_HIFB: u32 = 1_074_022_027;
pub const FBIOGET_DYNAMIC_RANGE_HIFB: u32 = 2_147_763_852;
pub const FBIO_RELEASE_HIFB: u32 = 18_070;
pub const FBIOPUT_CURSOR_INFO: u32 = 1_076_905_576;
pub const FBIOGET_CURSOR_INFO: u32 = 1_076_905_577;
pub const FBIOPUT_CURSOR_STATE: u32 = 1_074_021_994;
pub const FBIOGET_CURSOR_STATE: u32 = 1_074_021_995;
pub const FBIOPUT_CURSOR_POS: u32 = 1_074_284_140;
pub const FBIOGET_CURSOR_POS: u32 = 2_148_025_965;
pub const FBIOPUT_CURSOR_COLORKEY: u32 = 2_148_025_966;
pub const FBIOGET_CURSOR_COLORKEY: u32 = 1_074_284_143;
pub const FBIOPUT_CURSOR_ALPHA: u32 = 2_148_288_112;
pub const FBIOGET_CURSOR_ALPHA: u32 = 1_074_546_289;
pub const FBIOPUT_CURSOR_ATTCHCURSOR: u32 = 1_074_022_002;
pub const FBIOPUT_CURSOR_DETACHCURSOR: u32 = 1_074_022_003;
pub const FBIOPUT_LAYER_INFO: u32 = 1_076_905_592;
pub const FBIOGET_LAYER_INFO: u32 = 2_150_647_417;
pub const FBIOGET_CANVAS_BUFFER: u32 = 2_151_171_707;
pub const FBIO_REFRESH: u32 = 1_077_429_884;
pub const FBIO_WAITFOR_FREFRESH_DONE: u32 = 18_045;
pub const FBIOPUT_MIRROR_MODE: u32 = 1_074_022_014;
pub const FBIOGET_MIRROR_MODE: u32 = 1_074_022_015;
pub const FBIOPUT_ROTATE_MODE: u32 = 1_074_022_016;
pub const FBIOGET_ROTATE_MODE: u32 = 1_074_022_017;
pub const FBIOPUT_MDDRDETECT_HIFB: u32 = 1_074_284_167;
pub const FBIOGET_MDDRDETECT_HIFB: u32 = 1_074_284_168;

/// Make HI_BOOL can convert to bool.
impl std::convert::Into<bool> for HI_BOOL {
    fn into(self) -> bool {
        self == HI_BOOL::HI_TRUE
    }
}

/// Make bool can convert to HI_BOOL.
impl std::convert::Into<HI_BOOL> for bool {
    fn into(self) -> HI_BOOL {
        if self {
            HI_BOOL::HI_TRUE
        } else {
            HI_BOOL::HI_FALSE
        }
    }
}

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
