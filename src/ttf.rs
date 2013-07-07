#[link(name = "sdl_ttf",
       vers = "0.1",
       uuid = "1d776704-431f-4d5b-9361-b27958535321",
       url = "http://github.com/sfackler/rust-sdl_ttf")];

#[comment="SDL_ttf bindings"];
#[license="MIT"];
#[crate_type="lib"];

extern mod sdl;

use std::cast;
use std::libc::{c_int, c_long};
use std::str;

use sdl::video::{Color, Surface};

mod ffi {
    use std::libc::{c_int, c_char, c_void, c_long};
    use sdl::video::ll::{SDL_Color, SDL_Surface};

    pub type TTF_Font = c_void;

    pub type TTF_StyleFlag = c_int;
    pub static TTF_STYLE_NORMAL: TTF_StyleFlag = 0x00;
    pub static TTF_STYLE_BOLD: TTF_StyleFlag = 0x01;
    pub static TTF_STYLE_ITALIC: TTF_StyleFlag = 0x02;
    pub static TTF_STYLE_UNDERLINE: c_int = 0x04;
    pub static TTF_STYLE_STRIKETHROUGH: c_int = 0x08;

    pub type TTF_Hinting = c_int;
    pub static TTF_HINTING_NORMAL: TTF_Hinting = 0;
    pub static TTF_HINTING_LIGHT: TTF_Hinting = 1;
    pub static TTF_HINTING_MONO: TTF_Hinting = 2;
    pub static TTF_HINTING_NONE: TTF_Hinting = 3;

    #[link_args = "-lSDL_ttf"]
    extern "C" {
        fn TTF_Init() -> c_int;
        fn TTF_WasInit() -> c_int;
        fn TTF_Quit();
        fn TTF_OpenFont(file: *c_char, ptsize: c_int) -> *TTF_Font;
        fn TTF_OpenFontIndex(file: *c_char, ptsize: c_int, index: c_long)
            -> *TTF_Font;
        fn TTF_CloseFont(font: *TTF_Font);

        fn TTF_GetFontStyle(font: *TTF_Font) -> TTF_StyleFlag;
        fn TTF_SetFontStyle(font: *TTF_Font, style: TTF_StyleFlag);
        fn TTF_GetFontOutline(font: *TTF_Font) -> c_int;
        fn TTF_SetFontOutline(font: *TTF_Font, outline: c_int);
        fn TTF_GetFontHinting(font: *TTF_Font) -> TTF_Hinting;
        fn TTF_SetFontHinting(font: *TTF_Font, hinting: TTF_Hinting);
        fn TTF_GetFontKerning(font: *TTF_Font) -> c_int;
        fn TTF_SetFontKerning(font: *TTF_Font, kerning: c_int);
        fn TTF_FontHeight(font: *TTF_Font) -> c_int;
        fn TTF_FontAscent(font: *TTF_Font) -> c_int;
        fn TTF_FontDescent(font: *TTF_Font) -> c_int;
        fn TTF_FontLineSkip(font: *TTF_Font) -> c_int;
        fn TTF_FontFaces(font: *TTF_Font) -> c_long;
        fn TTF_FontFaceIsFixedWidth(font: *TTF_Font) -> c_int;
        fn TTF_FontFaceFamilyName(font: *TTF_Font) -> *c_char;
        fn TTF_FontGlyphIsProvided(font: *TTF_Font, glyph: u16) -> c_int;
        fn TTF_GlyphMetrics(font: *TTF_Font, glyph: u16, minx: *mut c_int,
            maxx: *mut c_int, miny: *mut c_int, maxy: *mut c_int,
            advance: *mut c_int) -> c_int;
        fn TTF_SizeUTF8(font: *TTF_Font, text: *c_char, w: *mut c_int,
            h: *mut c_int) -> c_int;

        fn TTF_RenderUTF8_Solid(font: *TTF_Font, text: *c_char,
            fg: SDL_Color) -> *SDL_Surface;
        fn TTF_RenderUTF8_Shaded(font: *TTF_Font, text: *c_char,
            fg: SDL_Color, bg: SDL_Color) -> *SDL_Surface;
        fn TTF_RenderUTF8_Blended(font: *TTF_Font, text: *c_char,
            fg: SDL_Color) -> *SDL_Surface;
    }
}

pub enum FontStyle {
    NormalStyle = ffi::TTF_STYLE_NORMAL as int,
    BoldStyle = ffi::TTF_STYLE_BOLD as int,
    ItalicStyle = ffi::TTF_STYLE_ITALIC as int,
    UnderlineStyle = ffi::TTF_STYLE_UNDERLINE as int,
    StrikethroughStyle = ffi::TTF_STYLE_STRIKETHROUGH as int
}

pub enum FontHinting {
    NormalHinting = ffi::TTF_HINTING_NORMAL as int,
    LightHinting = ffi::TTF_HINTING_LIGHT as int,
    MonoHinting = ffi::TTF_HINTING_MONO as int,
    NoneHinting = ffi::TTF_HINTING_NONE as int
}

pub struct GlyphMetrics {
    minx: int,
    maxx: int,
    miny: int,
    maxy: int,
    advance: int
}

pub struct Font {
    priv raw: *ffi::TTF_Font
}

impl Drop for Font {
    fn drop(&self) {
        unsafe {
            ffi::TTF_CloseFont(self.raw);
        }
    }
}

impl Font {
    pub fn get_style(&self) -> ~[FontStyle] {
        let bitflags = unsafe { ffi::TTF_GetFontStyle(self.raw) };

        let flags = [
            NormalStyle,
            BoldStyle,
            ItalicStyle,
            UnderlineStyle,
            StrikethroughStyle
        ];

        do flags.iter().filter_map |&flag| {
            if bitflags & (flag as ffi::TTF_StyleFlag) != 0 { Some(flag) }
            else {None}
        }.collect()
    }

    pub fn set_style(&mut self, flags: &[FontStyle]) {
        let bitflags = do flags.iter().fold(0) |bitflags, &flag| {
            bitflags | flag as ffi::TTF_StyleFlag
        };

        unsafe {
            ffi::TTF_SetFontStyle(self.raw, bitflags);
        }
    }

    pub fn get_outline(&self) -> int {
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as int
        }
    }

    pub fn set_outline(&mut self, outline: int) {
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int);
        }
    }

    pub fn get_hinting(&self) -> FontHinting {
        unsafe {
            cast::transmute(ffi::TTF_GetFontHinting(self.raw) as int)
        }
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) {
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as ffi::TTF_Hinting);
        }
    }

    pub fn get_kerning(&self) -> int {
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) as int
        }
    }

    pub fn set_kerning(&mut self, kerning: int) {
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int);
        }
    }

    pub fn height(&self) -> int {
        unsafe {
            ffi::TTF_FontHeight(self.raw) as int
        }
    }

    pub fn ascent(&self) -> int {
        unsafe {
            ffi::TTF_FontAscent(self.raw) as int
        }
    }

    pub fn descent(&self) -> int {
        unsafe {
            ffi::TTF_FontDescent(self.raw) as int
        }
    }

    pub fn line_skip(&self) -> int {
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as int
        }
    }

    pub fn faces(&self) -> int {
        unsafe {
            ffi::TTF_FontFaces(self.raw) as int
        }
    }

    pub fn face_is_fixed_width(&self) -> bool {
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) > 0
        }
    }

    pub fn face_family_name(&self) -> Option<~str> {
        unsafe {
            let ptr = ffi::TTF_FontFaceFamilyName(self.raw);

            if ptr.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(ptr))
            }
        }
    }

    pub fn glyph_is_provided(&self, glyph: char) -> Option<int> {
        let ch = match char_to_utf16(glyph) {
            Some(ch) => ch,
            None => return None
        };

        unsafe {
            match ffi::TTF_FontGlyphIsProvided(self.raw, ch) {
                0 => None,
                ch => Some(ch as int)
            }
        }
    }

    pub fn glyph_metrics(&self, glyph: char) -> Result<~GlyphMetrics, ~str> {
        let ch = match char_to_utf16(glyph) {
            Some(ch) => ch,
            None => return Err(~"Glyph is not a UTF-16 character")
        };

        let mut minx: c_int = 0;
        let mut maxx: c_int = 0;
        let mut miny: c_int = 0;
        let mut maxy: c_int = 0;
        let mut advance: c_int = 0;

        unsafe {
            match ffi::TTF_GlyphMetrics(self.raw, ch, &mut minx, &mut maxx,
                    &mut miny, &mut maxy, &mut advance) {
                0 => Ok(~GlyphMetrics {minx: minx as int, maxx: maxx as int,
                    miny: miny as int, maxy: maxy as int,
                    advance: advance as int}),
                _ => Err(sdl::get_error())
            }
        }
    }

    pub fn text_size(&self, text: &str) -> Result<(int, int), ~str> {
        let mut w: c_int = 0;
        let mut h: c_int = 0;

        do text.as_c_str |c_text| {
            unsafe {
                match ffi::TTF_SizeUTF8(self.raw, c_text, &mut w, &mut h) {
                    0 => Ok((w as int, h as int)),
                    _ => Err(sdl::get_error())
                }
            }
        }
    }
}

fn char_to_utf16(glyph: char) -> Option<u16> {
    // TODO bounds checking
    if !str::is_utf16([glyph as u16]) {
        return None;
    }

    Some(str::from_char(glyph).to_utf16()[0])
}

pub fn init() -> bool {
    unsafe {
        ffi::TTF_Init() == 0
    }
}

pub fn was_init() -> bool {
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}

pub fn quit() {
    unsafe {
        ffi::TTF_Quit();
    }
}

pub fn open_font(file: &str, ptsize: int) -> Result<~Font, ~str> {
    do file.as_c_str |c_str| {
        unsafe {
            let ptr = ffi::TTF_OpenFont(c_str, ptsize as c_int);
            if ptr.is_null() {
                Err(sdl::get_error())
            } else {
                Ok(~Font {raw: ptr})
            }
        }
    }
}

pub fn open_font_index(file: &str, ptsize: int, index: int)
        -> Result<~Font, ~str> {
    do file.as_c_str |c_str| {
        let ptr = unsafe {
            ffi::TTF_OpenFontIndex(c_str, ptsize as c_int, index as c_long)
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Font {raw: ptr})
        }
    }
}

pub fn render_solid(font: &Font, text: &str, fg: Color)
        -> Result<~Surface, ~str> {
    do text.as_c_str |c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Solid(font.raw, c_text, fg.to_struct())
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    }
}

pub fn render_shaded(font: &Font, text: &str, fg: Color, bg: Color)
        -> Result<~Surface, ~str> {
    do text.as_c_str |c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Shaded(font.raw, c_text, fg.to_struct(),
                bg.to_struct())
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    }
}

pub fn render_blended(font: &Font, text: &str, fg: Color)
        -> Result<~Surface, ~str> {
    do text.as_c_str |c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Blended(font.raw, c_text, fg.to_struct())
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    }
}
