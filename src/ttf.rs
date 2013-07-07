#[link(name = "sdl_ttf",
       vers = "0.1",
       uuid = "1d776704-431f-4d5b-9361-b27958535321",
       url = "http://github.com/sfackler/rust-sdl_ttf")];

#[comment="SDL_ttf bindings"];
#[license="MIT"];
#[crate_type="lib"];

extern mod sdl;

use std::libc::{c_int, c_long};
use std::ptr;

use sdl::video::{Color, Surface};

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link_args="-lSDL_ttf"]
    extern {}
}

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

    extern {
        fn TTF_Init() -> c_int;
        fn TTF_WasInit() -> c_int;
        fn TTF_Quit();
        fn TTF_OpenFont(file: *c_char, ptsize: c_int) -> *TTF_Font;
        fn TTF_OpenFontIndex(file: *c_char, ptsize: c_int, index: c_long)
            -> *TTF_Font;
        fn TTF_CloseFont(font: *TTF_Font);

        fn TTF_GetFontStyle(font: *TTF_Font) -> c_int;
        fn TTF_SetFontStyle(font: *TTF_Font, style: c_int);
        fn TTF_GetFontOutline(font: *TTF_Font) -> c_int;
        fn TTF_SetFontOutline(font: *TTF_Font, outline: c_int);

        fn TTF_RenderText_Solid(font: *TTF_Font, text: *c_char,
            color: SDL_Color) -> *SDL_Surface;
    }
}

pub enum FontStyle {
    Normal = ffi::TTF_STYLE_NORMAL as int,
    Bold = ffi::TTF_STYLE_BOLD as int,
    Italic = ffi::TTF_STYLE_ITALIC as int,
    Underline = ffi::TTF_STYLE_UNDERLINE as int,
    Strikethrough = ffi::TTF_STYLE_STRIKETHROUGH as int
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
            Normal,
            Bold,
            Italic,
            Underline,
            Strikethrough
        ];

        do flags.iter().filter_map |&flag| {
            if bitflags & (flag as ffi::TTF_StyleFlag) != 0 { Some(flag) }
            else {None}
        }.collect()
    }

    pub fn set_style(&self, flags: &[FontStyle]) {
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

    pub fn set_outline(&self, outline: int) {
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int);
        }
    }
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
            if ptr == ptr::null() {
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

        if ptr == ptr::null() {
            Err(sdl::get_error())
        } else {
            Ok(~Font {raw: ptr})
        }
    }
}

pub fn render_text_solid(font: &Font, text: &str, color: Color)
        -> Result<~Surface, ~str> {
    do text.as_c_str |c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderText_Solid(font.raw, c_text, color.to_struct())
        };

        if ptr == ptr::null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    }
}
