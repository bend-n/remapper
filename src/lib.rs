#![allow(incomplete_features, internal_features)]
#![feature(
    isqrt,
    const_fn_floating_point_arithmetic,
    inline_const_pat,
    iter_chain,
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
use fimg::{indexed::IndexedImage, Image};
use kd::KD;
// type KD = kiddo::immutable::float::kdtree::ImmutableKdTree<f32, u64, 4, 32>;
fn map(colors: &[[f32; 4]]) -> KD {
    KD::new(colors)
}

fn dither<'a>(
    image: Image<&[f32], 4>,
    f: impl FnMut(((usize, usize), &[f32; 4])) -> u32,
    pal: &'a [[f32; 4]],
) -> IndexedImage<Box<[u32]>, &'a [[f32; 4]]> {
    IndexedImage::build(image.width(), image.height())
        .pal(pal)
        .buf(
            image
                .chunked()
                .zip(image.ordered())
                .map(|(p, xy)| (xy.array().map(|x| x as usize).tuple(), p))
                .map(f)
                .collect(),
        )
}
