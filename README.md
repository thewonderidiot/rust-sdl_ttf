# Rust-SDL_ttf
SDL_ttf bindings for Rust.

# Requirements

* Rust - Rust-SDL_ttf builds against the `master` branch of the [Rust
    development repository](http://github.com/mozilla/rust). It will probably
    not compile against the stable releases from http://rust-lang.org.

* rust-sdl - This provides bindings to the base SDL library, available at
    http://github.com/brson/rust-sdl.

* SDL_ttf - Available through package management tools or
    http://www.libsdl.org.

# Installation

    $ git clone https://github.com/sfackler/rust-sdl_ttf.git
    $ cd rust-sdl_ttf
    $ rustpkg install sdl_ttf

You may have to tell rustc where SDL_ttf and/or rust-sdl are located. E.g.:

    $ RUSTFLAGS="-L~/rust-sdl -L/usr/local/lib" make
