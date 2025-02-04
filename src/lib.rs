#![allow(incomplete_features, internal_features, mixed_script_confusables)]
#![feature(
    custom_inner_attributes,
    proc_macro_hygiene,
    vec_into_raw_parts,
    type_alias_impl_trait,
    inline_const_pat,
    iter_chain,
    adt_const_params,
    stmt_expr_attributes,
    iter_array_chunks,
    let_chains,
    generic_const_exprs,
    core_intrinsics,
    iter_intersperse,
    maybe_uninit_array_assume_init,
    array_windows,
    iter_map_windows
)]
#![allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct pal<'palette, const N: usize> {
    inner: &'palette [[f32; N]],
}
impl<'a, const N: usize> pal<'a, N> {
    /// Create a ne palette. The length can not be 0 and must be < u32::MAX.
    pub fn new(value: &'a [[f32; N]]) -> Self {
        let value = value.as_ref();
        assert!(value.len() != 0);
        assert!(value.len() < u32::MAX as usize);
        pal { inner: value }
    }
}

impl<'a, const N: usize> AsRef<[[f32; N]]> for pal<'a, N> {
    fn as_ref(&self) -> &[[f32; N]] {
        &*self
    }
}
impl<'a, const N: usize> Deref for pal<'a, N> {
    type Target = [[f32; N]];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
type out<'palette, P> = IndexedImage<Box<[u32]>, P>;

pub mod diffusion;
pub mod ordered;

pub mod dumb;
use std::ops::Deref;

use atools::prelude::*;
use dumb::Closest;
use fimg::{indexed::IndexedImage, Image};

fn dither<'a, const C: usize>(
    image: Image<impl AsRef<[f32]>, C>,
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
