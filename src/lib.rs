#![feature(
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
use exoquant::Remapper;
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
            .flat_map(|(x, p)| p.iter().enumerate().map(move |(y, p)| ((x % 2, y % 2), p)))
            .flat_map(f)
            .collect(),
    )
}

pub fn remap_bayer_2x2(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_2X2[x + y * 2]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_4x4(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_4X4[x + y * 4]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_8x8(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither(image, |((x, y), &p)| {
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
