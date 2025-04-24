# image-gameboy

[![Crates.io](https://img.shields.io/crates/v/image-gameboy.svg)](https://crates.io/crates/image-gameboy)
[![Docs.rs](https://docs.rs/image-gameboy/badge.svg)](https://docs.rs/image-gameboy)

A simple Rust crate for converting images into the Game Boy's 2 bits-per-pixel (2bpp) tile format.
Built to work seamlessly with the [`image`](https://crates.io/crates/image) crate.

## Features

- 🧩 Provides the `into_gb2bpp()` extension method on `DynamicImage`
- 🎮 Outputs binary tile data in the Game Boy-compatible 2bpp format
- 🖼️ Supports grayscale images with dimensions divisible by 8
- 🔗 Easy to integrate in asset pipelines for Game Boy homebrew or ROM hacking

## Usage

### Add to your `Cargo.toml`

```toml
[dependencies]
image = "0.25"
image-gameboy = "0.1"
```

## Example

```rust
use image::io::Reader as ImageReader;
use image_gameboy::GameBoy2bpp;

fn main() {
    let img = ImageReader::open("assets/sprite.png")
        .unwrap()
        .decode()
        .unwrap();

    let gb_data = img.into_gb2bpp();

    std::fs::write("sprite.2bpp", gb_data).unwrap();
}
```

## What is Game Boy 2bpp?

The original Game Boy used a tile-based graphics system where each 8×8 tile is represented by 16 bytes:
- 2 bytes per row (8 rows total)
- Each pixel uses 2 bits (4 grayscale levels)
- The bits are split across two “bitplanes” (least and most significant bits)

This crate generates exactly that format from any grayscale image.

## Notes
- Input images must be grayscale and have dimensions divisible by 8
- Each pixel is quantized to one of 4 grayscale levels:
    - 0..=63 → 3 (darkest)
    - 64..=127 → 2
    - 128..=191 → 1
    - 192..=255 → 0 (lightest)

## License

Licensed under the MIT License or Apache 2.0.
