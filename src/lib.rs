#![feature(
    adt_const_params,
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
use dumb::Closest;
use exoquant::Remapper;
use fimg::Image;
use kd::KD;
// type KD = kiddo::immutable::float::kdtree::ImmutableKdTree<f32, u64, 4, 32>;
fn map(colors: &[[f32; 4]]) -> KD {
    KD::new(colors)
}

pub fn remap(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    Image::build(image.width(), image.height()).buf(
        image
            .chunked()
            .map(|x| palette[kd.find_nearest(*x) as usize])
            // .map(|&x| palette.closest(x).1)
            .flatten()
            .collect(),
    )
}
