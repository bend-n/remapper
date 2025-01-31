#![allow(incomplete_features, internal_features, mixed_script_confusables)]
#![feature(
    isqrt,
    vec_into_raw_parts,
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
#![allow(non_camel_case_types)]
type pal<'palette, const N: usize> = &'palette [[f32; N]];
type out<'palette, P> = IndexedImage<Box<[u32]>, P>;

pub mod diffusion;
pub mod ordered;

pub mod dumb;
use atools::prelude::*;
use dumb::Closest;
use fimg::{indexed::IndexedImage, Image};

fn dither<'a, const C: usize>(
    image: Image<&[f32], C>,
    f: impl FnMut(((usize, usize), &[f32; C])) -> u32,
    pal: pal<'a, C>,
) -> out<'a, pal<'a, C>> {
    unsafe {
        IndexedImage::build(image.width(), image.height())
            .pal(pal)
            .buf_unchecked(
                image
                    .chunked()
                    .zip(image.ordered())
                    .map(|(p, xy)| (xy.array().map(|x| x as usize).tuple(), p))
                    .map(f)
                    .collect(),
            )
    }
}
