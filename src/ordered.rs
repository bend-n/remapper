//! # Ordered dithering.
//! The way this works is by adding a constant texture to the image, and then quantizing that.
use fimg::indexed::IndexedImage;

use super::*;

const fn threshold<const N: usize>(x: [u32; N]) -> [f32; N] {
    car::map!(x, |x| x as f32 * (1. / N as f32)
        - 0.5 * ((N - 1) as f32 * (1. / N as f32)))
}

const fn next<const N: usize>(input: [u32; N]) -> [u32; N.isqrt() * 2 * N.isqrt() * 2]
where
    [(); N.isqrt() * N.isqrt()]:,
{
    next_::<{ N.isqrt() }>(unsafe { std::intrinsics::transmute_unchecked(input) })
}

// https://github.com/surma/surma.dev/blob/master/static/lab/ditherpunk/bayer-worker.js#L6C1-L23C1
const fn next_<const N: usize>(input: [u32; N * N]) -> [u32; N * 2 * N * 2] {
    let mut output = [0; { N * 2 * N * 2 }];
    let base = [0, 2, 3, 1];
    let mut y = 0;
    while y != N * 2 {
        let mut x = 0;
        while x != N * 2 {
            output[y * N * 2 + x] = 4 * input[(y % N) * N + (x % N)]
                + base[((y >= N) as usize) * 2 + ((x >= N) as usize)];
            x += 1;
        }
        y += 1;
    }
    output
}
const BAYER0: [u32; 4] = [
    0, 2, //
    3, 1,
];
const BAYER1: [u32; 4 * 4] = next(BAYER0);
const BAYER2: [u32; 8 * 8] = next(BAYER1);
const BAYER3: [u32; 16 * 16] = next(BAYER2);
const BAYER4: [u32; 32 * 32] = next(BAYER3);
const BAYER5: [u32; 64 * 64] = next(BAYER4);

const BAYER_2X2: [f32; 4] = threshold(BAYER0);
const BAYER_4X4: [f32; 4 * 4] = threshold(BAYER1);
const BAYER_8X8: [f32; 8 * 8] = threshold(BAYER2);
const BAYER_16X16: [f32; 16 * 16] = threshold(BAYER3);
const BAYER_32X32: [f32; 32 * 32] = threshold(BAYER4);
const BAYER_64X64: [f32; 64 * 64] = threshold(BAYER5);

fn dither_with<'a, const N: usize>(
    image: Image<&[f32], 4>,
    mut f: impl FnMut(((usize, usize), &[f32; 4])) -> u32,
    palette: &'a [[f32; 4]],
) -> IndexedImage<Box<[u32]>, &'a [[f32; 4]]> {
    dither(image, |((x, y), p)| f(((x % N, y % N), p)), palette)
}

macro_rules! bayer {
    ($i:ident, $c:ident, $j:literal) => {
        /// Ordered dithering via a bayer matrix.
        ///
        /// Dont expect too much difference from each of them.
        pub fn $i<'a>(
            image: Image<&[f32], 4>,
            palette: &'a [[f32; 4]],
        ) -> IndexedImage<Box<[u32]>, &'a [[f32; 4]]> {
            let kd = map(palette);
            let r = kd.space(palette);
            dither_with::<$j>(
                image.into(),
                |((x, y), &p)| {
                    let color = p.add(r * $c[x + y * $j]);
                    kd.find_nearest(color)
                },
                palette,
            )
        }
    };
}

bayer!(bayer2x2, BAYER_2X2, 2);
bayer!(bayer4x4, BAYER_4X4, 4);
bayer!(bayer8x8, BAYER_8X8, 8);
bayer!(bayer16x16, BAYER_16X16, 16);
bayer!(bayer32x32, BAYER_32X32, 32);
bayer!(bayer64x64, BAYER_64X64, 64);

pub fn remap<'a, 'b>(
    image: Image<&'b [f32], 4>,
    palette: &'a [[f32; 4]],
) -> IndexedImage<Box<[u32]>, &'a [[f32; 4]]> {
    let kd = map(palette);
    // todo!();
    IndexedImage::build(image.width(), image.height())
        .pal(palette)
        .buf(image.chunked().map(|x| kd.find_nearest(*x)).collect())
}

const BLUE: Image<[f32; 1024 * 1024 * 3], 3> = unsafe {
    Image::new(
        std::num::NonZero::new(1024).unwrap(),
        std::num::NonZero::new(1024).unwrap(),
        std::mem::transmute(*include_bytes!("../blue.f32")),
    )
};
// todo: figure this out? seems off.
/*
pub fn remap_blue(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    // Image::<Box<[u8]>, 3>::from(BLUE.as_ref()).show();
    dither(image, |((x, y), p)| {
        let (p, al) = p.pop();
        let noise = unsafe { BLUE.pixel(x as u32 % 1024, y as u32 % 1024) }.sub(0.5);

        fn lin_to_srgb(x: f32) -> f32 {
            if x.abs() <= 0.0031308 {
                x * 12.92
            } else {
                (1.055 * x.abs().powf(1.0 / 2.4) - 0.055).copysign(x)
            }
        }

        fn srgb_to_lin(x: f32) -> f32 {
            if x.abs() <= 0.04045 {
                x * (1.0 / 12.92)
            } else {
                ((x.abs() + 0.055) * (1.0 / 1.055)).powf(2.4).copysign(x)
            }
        }

        let c = p
            .map(srgb_to_lin)
            .zip(noise)
            .map(|(x, noise)| x + noise)
            .map(lin_to_srgb)
            .join(al);
        // let yuv = [
        //     p.amul([0.299, 0.587, 0.114]).sum(),
        //     p.amul([-0.14713, -0.28886, 0.436]).sum(),
        //     p.amul([0.615, -0.51499, -0.10001]).sum(),
        // ];
        // let c = yuv.zip(noise).map(|(x, noise)| x + noise);
        // let c = [
        //     c.amul([1., 0., 1.13983]).sum(),
        //     c.amul([1., -0.39465, -0.58060]).sum(),
        //     c.amul([1., 2.03211, 0.]).sum(),
        // ];

        // let c = c.join(al);
        palette[kd.find_nearest(c) as usize]
    })
}



pub fn remap_triangular(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
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
                        (2.0 * noise).sqrt() - 1.0
                    } else {
                        1.0 - (2.0 - 2.0 * noise).sqrt()
                    }
                };
                x + noise
            })
            .join(al);
        palette[kd.find_nearest(c) as usize]
    })
}
*/
