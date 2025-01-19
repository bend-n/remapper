use fimg::{indexed::IndexedImage, Image};

fn test(
    k: &'static str,
    f: for<'a> fn(Image<&[f32], 4>, &'a [[f32; 4]]) -> IndexedImage<Box<[u32]>, &'a [[f32; 4]]>,
) {
    let pal = fimg::Image::<Box<[f32]>, 4>::from(fimg::Image::open("tdata/endesga.png").as_ref());
    let pal = pal.flatten();
    // let d = f(fimg::Image::open("tdata/small_cat.png").to_f32().as_ref(), &pal).to().to_u8().show();
    let d = f(
        fimg::Image::open("tdata/small_cat.png").to_f32().as_ref(),
        &pal,
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
            test(stringify!($x), $call);
        }
    };
}

test!(o2x2, remapper::ordered::bayer2x2);
test!(o4x4, remapper::ordered::bayer4x4);
test!(o8x8, remapper::ordered::bayer8x8);
test!(o16x16, remapper::ordered::bayer16x16);
test!(o32x32, remapper::ordered::bayer32x32);
test!(o64x64, remapper::ordered::bayer64x64);

// test!(s1, remapper::diffusion::sierra::sierra::<241>);
// test!(s2, remapper::diffusion::sierra::sierra_two::<241>);
// test!(s3, remapper::diffusion::sierra::sierra_lite::<241>);
