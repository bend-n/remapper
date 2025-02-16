#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use fimg::{indexed::IndexedImage, Image};
use remapper::pal;

fn test(
    k: &'static str,
    f: for<'a, 'b> fn(Image<&'b [f32], 4>, pal<'a, 4>) -> IndexedImage<Box<[u32]>, pal<'a, 4>>,
) {
    let pal = fimg::Image::<Box<[f32]>, 4>::from(fimg::Image::open("tdata/endesga.png").as_ref());
    let pal = pal.flatten();
    // let d = f(fimg::Image::open("tdata/small_cat.png").to_f32().as_ref(), &pal).to().to_u8().show();
    let d = f(
        fimg::Image::open("tdata/small_cat.png").to_f32().as_ref(),
        pal::new(pal),
    )
    .into_raw_parts()
    .0
    .take_buffer()
    .iter()
    .map(|&x| x as u8)
    .collect::<Vec<_>>();
    match std::fs::read(format!("tests/tres/{k}")) {
        Ok(x) => assert!(x == d, "{k}!  failed."),
        Err(_) => std::fs::write(format!("tests/tres/{k}"), d).unwrap(),
    }
}
macro_rules! test {
    ($x:ident, $call:path) => {
        #[test]
        fn $x() {
            test(stringify!($x), |x, p| $call(x, p));
        }
    };
    (boxed $x:ident, $call:path) => {
        #[test]
        fn $x() {
            test(stringify!($x), |x, p| $call(x.boxed(), p))
        }
    };
}

test!(o2x2, remapper::ordered::bayer2x2);
test!(o4x4, remapper::ordered::bayer4x4);
test!(o8x8, remapper::ordered::bayer8x8);
test!(o16x16, remapper::ordered::bayer16x16);
test!(o32x32, remapper::ordered::bayer32x32);
test!(o64x64, remapper::ordered::bayer64x64);

test!(boxed s1, remapper::diffusion::sierra::sierra::<255, 4>);
test!(boxed s2, remapper::diffusion::sierra::sierra_two::<255, 4>);
test!(boxed s3, remapper::diffusion::sierra::sierra_lite::<255, 4>);
