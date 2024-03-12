use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// This is equal to the [.2dj](https://docs.sc3.io/features/sc-peripherals.html#_2dj-format) format for sc-peripherals' wide format printer
pub struct Poster {
    /// The name of the poster, max 48 characters
    pub label: Option<String>,
    /// The tooltip of the poster in the inventory, max 256 characters
    pub tooltip: Option<String>,
    /// Up to 63 colors encoded as 0xRRGGBB integers, the transparent index 0 should not be included
    pub palette: Vec<u32>,
    /// 16384 (128*128) array of palette indices. 0 is transparent,
    /// 1 is the first palette color, etc.
    pub pixels: Vec<u16>,
    /// Width of the poster in pixels, not currently used
    pub width: u8,
    /// Height of the poster in pixels, not currently used
    pub height: u8,
}

#[derive(Debug, Deserialize, Serialize)]
/// This is equal to the [.2dja](https://docs.sc3.io/features/sc-peripherals.html#_2dja-format) format for sc-peripherals' wide format printer
pub struct Posters {
    /// The name of the collection of posters, not currently used
    pub title: Option<String>,
    /// Count of posters in a row, not currently used
    pub width: Option<u8>,
    /// Count of posters in a column, not currently used
    pub height: Option<u8>,
    /// Array of 2dj files, length should be width * height if specified
    pub pages: Vec<Poster>,
}