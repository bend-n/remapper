//! # Error diffusion dithering.
//! The way this works is by finding the amount of error between the quantized color and the original color, and offseting the error to the (next) neighboring pixels.
//! Which neighboring pixels depend on the algorithm chosen.
use super::*;
mod riemerasma;
pub mod sierra;
pub use riemerasma::*;
pub fn atkinson(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    let eighth = [1. / 8.; 4];
    for (x, y) in image.serpent() {
        unsafe {
            /*
              * 1 1
            1 1 1
              1
            */
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |x: [f32; 4]| x.aadd(error.amul(eighth));
            image.replace(x + 1, y, f);
            image.replace(x + 2, y, f);

            image.replace(x.wrapping_sub(1), y + 1, f);
            image.replace(x, y + 1, f);
            image.replace(x + 1, y + 1, f);

            image.replace(x, y + 2, f);
        }
    }
    image
}

pub fn jarvis<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    for (x, y) in image.serpent() {
        #[rustfmt::skip]
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| {
                move |x: [f32; 4]| {
                    x.aadd(error.amul([(f as f32 / 48.) * const { FAC as f32 * (1.0 / 255.) }; 4]))
                }
            };
            /*  * 7 5
            3 5 7 5 3
            1 3 5 3 1*/
            image.replace(x +             1, y, f(7));
            image.replace(x +             2, y, f(5));
            let y = y + 1;
            image.replace(x.wrapping_sub(2), y, f(3));
            image.replace(x.wrapping_sub(1), y, f(5));
            image.replace(x                , y, f(7));
            image.replace(x             + 1, y, f(5));
            image.replace(x             + 2, y, f(3));
            let y = y + 1;
            image.replace(x.wrapping_sub(2), y, f(1));
            image.replace(x.wrapping_sub(1), y, f(3));
            image.replace(x                , y, f(5));
            image.replace(x             + 1, y, f(3));
            image.replace(x             + 2, y, f(1));
        }
    }
    image
}
pub fn floyd_steinberg<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    for (x, y) in image.serpent() {
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| {
                move |x: [f32; 4]| {
                    x.aadd(error.amul([(f as f32 / 48.) * const { FAC as f32 * (1.0 / 255.) }; 4]))
                }
            };
            /*
              * 7
            3 5 1 */
            image.replace(x + 1, y, f(7));
            image.replace(x.wrapping_sub(1), y + 1, f(3));
            image.replace(x, y + 1, f(5));
            image.replace(x + 1, y + 1, f(1));
        }
    }
    image
}
