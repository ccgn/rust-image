//! This crate provides native rust implementations of
//! Image encoders and decoders and basic image manipulation
//! functions.

#![crate_name = "image"]
#![crate_type = "rlib"]

#![warn(missing_doc)]
#![warn(unnecessary_qualification)]
#![warn(unnecessary_typecast)]
#![feature(macro_rules)]

extern crate flate;

pub use color::ColorType as ColorType;

pub use color:: {
    Grey,
    RGB,
    Palette,
    GreyA,
    RGBA,

    Pixel,

    Luma,
    LumaA,
    Rgb,
    Rgba,
};

pub use image::ImageDecoder as ImageDecoder;
pub use image::ImageError as ImageError;
pub use image::ImageResult as ImageResult;
pub use image::ImageFormat as ImageFormat;
pub use imageops::FilterType as FilterType;

pub use imageops:: {
    Triangle,
    Nearest,
    CatmullRom,
    Gaussian,
    Lanczos3
};

pub use image:: {
    PNG,
    JPEG,
    GIF,
    WEBP,
    PPM
};

//Image Types
pub use image::SubImage as SubImage;
pub use image::ImageBuf as ImageBuf;
pub use dynimage::DynamicImage as DynamicImage;

//Traits
pub use image::GenericImage as GenericImage;
pub use image::MutableRefImage as MutableRefImage;

//Iterators
pub use image::Pixels as Pixels;
pub use image::MutPixels as MutPixels;

///opening and loading images
pub use dynimage:: {
    open,
    load,
    load_from_memory,

    ImageRgb8,
    ImageRgba8,
    ImageLuma8,
    ImageLumaA8,
};

//Image Processing Functions
pub mod imageops;

//Image Codecs
pub mod webp;
pub mod ppm;
pub mod png;
pub mod jpeg;
pub mod gif;

mod image;
mod dynimage;
mod color;