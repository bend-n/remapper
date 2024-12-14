#![feature(
    const_option,
    adt_const_params,
    iter_array_chunks,
    let_chains,
    effects,
    const_refs_to_cell,
    generic_const_exprs,
    core_intrinsics,
    iter_intersperse,
    const_trait_impl,
    maybe_uninit_array_assume_init,
    array_windows,
    iter_map_windows
)]
mod dumb;
mod kd;
use atools::prelude::*;
use dumb::Closest;
use fimg::Image;
use kd::KD;
// type KD = kiddo::immutable::float::kdtree::ImmutableKdTree<f32, u64, 4, 32>;
fn map(colors: &[[f32; 4]]) -> KD {
    KD::new(colors)
}

static BAYER_2X2: [f32; 4] = {
    let map = [
        0, 2, //
        3, 1,
    ];
    car::map!(map, |x| x as f32 * (1. / 4.) - 0.5 * (3. * (1. / 4.)))
};
static BAYER_4X4: [f32; 4 * 4] = {
    let map = [
        0, 8, 2, 10, //
        12, 4, 14, 6, //
        3, 11, 1, 9, //
        15, 7, 13, 5,
    ];
    car::map!(map, |x| x as f32 * (1. / 16.) - 0.5 * (15. * (1. / 16.)))
};
static BAYER_8X8: [f32; 8 * 8] = {
    let map = [
        0, 32, 8, 40, 2, 34, 10, 42, //
        48, 16, 56, 24, 50, 18, 58, 26, //
        12, 44, 4, 36, 14, 46, 6, 38, //
        60, 28, 52, 20, 62, 30, 54, 22, //
        3, 35, 11, 43, 1, 33, 9, 41, //
        51, 19, 59, 27, 49, 17, 57, 25, //
        15, 47, 7, 39, 13, 45, 5, 37, //
        63, 31, 55, 23, 61, 29, 53, 21, //
    ];
    car::map!(map, |x| x as f32 * (1. / 64.) - 0.5 * (63. * (1. / 64.)))
};

fn dither(
    image: Image<&[f32], 4>,
    f: impl FnMut(((usize, usize), &[f32; 4])) -> [f32; 4],
) -> Image<Box<[f32]>, 4> {
    Image::build(image.width(), image.height()).buf(
        image
            .rows()
            .enumerate()
            .flat_map(|(x, p)| p.iter().enumerate().map(move |(y, p)| ((x, y), p)))
            .flat_map(f)
            .collect(),
    )
}

fn dither_with<const N: usize>(
    image: Image<&[f32], 4>,
    mut f: impl FnMut(((usize, usize), &[f32; 4])) -> [f32; 4],
) -> Image<Box<[f32]>, 4> {
    dither(image, |((x, y), p)| f(((x % N, y % N), p)))
}

pub fn remap_triangular(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    const BLUE: Image<[f32; 1024 * 1024 * 3], 3> = unsafe {
        Image::new(
            std::num::NonZero::new(1024).unwrap(),
            std::num::NonZero::new(1024).unwrap(),
            std::mem::transmute(*include_bytes!("../blue.f32")),
        )
    };
    dither(image, |((x, y), p)| {
        let (p, al) = p.pop();
        let noise = unsafe { BLUE.pixel(x as u32 % 1024, y as u32 % 1024) };
        let c = p
            .zip(noise)
            .map(|(x, noise)| {
                let noise = if x < (0.5 / 255.) || x > (254.5 / 255.) {
                    noise - 0.5
                } else {
                    if noise < 0.5 {
                        (2.0 * (noise)).sqrt() - 1.0
                    } else {
                        1.0 - (2.0 - 2.0 * noise).sqrt()
                    }
                };
                x + noise - 0.2
            })
            .join(al);
        palette[kd.find_nearest(c) as usize]
    })
}

pub fn remap_floyd_steinberg<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    let w = image.width();
    let h = image.height();
    let fac = FAC as f32 * (1.0 / 255.0);
    let 七 = 7. / 16. * fac;
    let 三 = 3. / 16. * fac;
    let 五 = 5. / 16. * fac;
    let 一 = 1. / 16. * fac;
    for (x, y) in (0..h).flat_map(move |y| (0..w).map(move |x| (x, y))) {
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| move |x: [f32; 4]| x.aadd(error.amul([f; 4]));
            image.replace(x + 1, y, f(七));
            image.replace(x.wrapping_sub(1), y + 1, f(三));
            image.replace(x, y + 1, f(五));
            image.replace(x + 1, y + 1, f(一));
        }
    }
    image
}

pub fn remap_atkinson(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    let w = image.width();
    let h = image.height();
    let eighth = [1. / 8.; 4];
    for (x, y) in (0..h).flat_map(move |y| (0..w).map(move |x| (x, y))) {
        unsafe {
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

pub fn remap_bayer_2x2(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<2>(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_2X2[x + y * 2]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_4x4(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<4>(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_4X4[x + y * 4]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_8x8(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<8>(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_8X8[x + y * 8]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    // todo!();
    Image::build(image.width(), image.height()).buf(
        image
            .chunked()
            .flat_map(|x| palette[kd.find_nearest(*x) as usize])
            // .map(|&x| palette.closest(x).1)
            .collect(),
    )
}
