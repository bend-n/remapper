//! # Error diffusion dithering.
//! The way this works is by finding the amount of error between the quantized color and the original color, and offseting the error to the (next) neighboring pixels.
//! Which neighboring pixels depend on the algorithm chosen.
use super::*;
mod riemerasma;
pub mod sierra;
pub use riemerasma::*;
pub fn atkinson<'a, const N: usize>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'a, N>,
) -> out<'a, pal<'a, N>> {
    let eighth = [1. / 8.; N];
    let out = out::build(image.width() as _, image.height() as _).pal(palette);
    let i = image.serpent().map(|(x, y)| unsafe {
        /*
          * 1 1
        1 1 1
          1
        */
        let p = image.pixel(x, y);
        let (_, new, index) = palette.closest(p);
        *image.pixel_mut(x, y) = new;
        let error = p.asub(new);
        let f = |x: [f32; N]| x.aadd(error.amul(eighth));
        image.replace(x + 1, y, f);
        image.replace(x + 2, y, f);

        image.replace(x.wrapping_sub(1), y + 1, f);
        image.replace(x, y + 1, f);
        image.replace(x + 1, y + 1, f);

        image.replace(x, y + 2, f);
        ((x, y), index as u32)
    });
    unsafe { out.from_iter(i) }
}

pub fn jarvis<'a, const N: usize, const FAC: u8>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'a, N>,
) -> out<'a, pal<'a, N>> {
    let out = out::build(image.width() as _, image.height() as _).pal(palette);
    #[rustfmt::skip]
    let i = image.serpent().map(|(x, y)| unsafe {
        let p = image.pixel(x, y);
        let (_, new, r) = palette.closest(p);
        *image.pixel_mut(x, y) = new;
        
        let error = p.asub(new);
        let f = |f| {
            move |x: [f32; N]| {
                x.aadd(error.amul([(f as f32 / 48.) * const { FAC as f32 * (1.0 / 255.) }; N]))
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
        ((x, y), r as u32)
    });
    unsafe { out.from_iter(i) }
}

pub fn floyd_steinberg<'p, const FAC: u8, const N: usize>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'p, N>,
) -> out<'p, pal<'p, N>> {
    let out = out::build(image.width() as _, image.height() as _).pal(palette);
    let i = image.serpent().map(|(x, y)| unsafe {
        let p = image.pixel(x, y);
        let (_, new, i) = palette.closest(p);
        let error = p.asub(new);
        let f = |f| {
            move |x: [f32; N]| {
                x.aadd(error.amul([(f as f32 / 48.) * const { FAC as f32 * (1.0 / 255.) }; N]))
            }
        };
        /*
          * 7
        3 5 1 */
        image.replace(x + 1, y, f(7));
        image.replace(x.wrapping_sub(1), y + 1, f(3));
        image.replace(x, y + 1, f(5));
        image.replace(x + 1, y + 1, f(1));

        ((x, y), i as u32)
    });
    unsafe { out.from_iter(i) }
}
