use image::DynamicImage;

pub trait GameBoy2bpp {
    fn into_gb2bpp(self) -> Vec<u8>;
}

impl GameBoy2bpp for DynamicImage {
    /// Converts the image into the Game Boy 2bpp (2 bits per pixel) format.
    ///
    /// This format is used for Game Boy graphics, where each pixel is represented
    /// by 2 bits, allowing for 4 shades of gray. The image is divided into 8x8 tiles,
    /// and each tile is encoded row by row. Each row consists of two bytes:
    /// - The first byte contains the least significant bits of the 8 pixels in the row.
    /// - The second byte contains the most significant bits of the 8 pixels in the row.
    ///
    /// The input image must have dimensions that are multiples of 8, as the Game Boy
    /// graphics system operates on 8x8 tiles.
    ///
    /// # Panics
    ///
    /// This function will panic if the input image dimensions are not multiples of 8.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the image data in Game Boy 2bpp format.
    fn into_gb2bpp(self) -> Vec<u8> {
        let img = self.into_luma8();
        let (width, height) = img.dimensions();

        assert!(
            width % 8 == 0 && height % 8 == 0,
            "Image dimensions must be a multiple of 8"
        );

        let mut result = Vec::new();

        for tile_y in 0..(height / 8) {
            for tile_x in 0..(width / 8) {
                for row in 0..8 {
                    let mut byte1 = 0u8;
                    let mut byte2 = 0u8;

                    for col in 0..8 {
                        let pixel = img.get_pixel(tile_x * 8 + col, tile_y * 8 + row)[0];
                        let value = match pixel {
                            0..=63 => 3,
                            64..=127 => 2,
                            128..=191 => 1,
                            _ => 0,
                        };

                        byte1 |= ((value & 1) as u8) << (7 - col);
                        byte2 |= ((value >> 1) as u8) << (7 - col);
                    }

                    result.push(byte1);
                    result.push(byte2);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::GameBoy2bpp;

    macro_rules! test_case {
        ($name:ident) => {
            #[test]
            fn $name() {
                let png_data = include_bytes!(concat!("../gfx/", stringify!($name), ".png"));
                let expected_bpp = include_bytes!(concat!("../gfx/", stringify!($name), ".2bpp"));

                let image = image::load_from_memory(png_data).unwrap();

                let actual = image.into_gb2bpp();
                assert_eq!(
                    actual,
                    expected_bpp,
                    "Converted data does not match for `{}`",
                    stringify!($name)
                );
            }
        };
    }

    test_case!(bird);
    test_case!(boulder);
    test_case!(cavern);
    test_case!(font_battle_extra);
    test_case!(font_extra);
    test_case!(red);
}
