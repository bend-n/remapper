#![allow(incomplete_features, internal_features)]
#![feature(
    inline_const_pat,
    const_option,
    adt_const_params,
    stmt_expr_attributes,
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

pub mod diffusion;
pub mod ordered;

mod dumb;
mod kd;
use atools::prelude::*;
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
