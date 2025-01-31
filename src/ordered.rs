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

fn dither_with<'a, const N: usize, const C: usize>(
    image: Image<&[f32], C>,
    mut f: impl FnMut(((usize, usize), &[f32; C])) -> u32,
    palette: pal<'a, C>,
) -> out<'a, pal<'a, C>> {
    dither(image, |((x, y), p)| f(((x % N, y % N), p)), palette)
}

macro_rules! bayer {
    ($i:ident, $c:ident, $j:literal) => {
        /// Ordered dithering via a bayer matrix.
        ///
        /// Dont expect too much difference from each of them.
        pub fn $i<'a, const C: usize>(
            image: Image<&[f32], C>,
            palette: pal<'a, C>,
        ) -> out<'a, pal<'a, C>> {
            let r = palette.space();
            dither_with::<$j, C>(
                image.into(),
                |((x, y), &p)| {
                    let color = p.add(r * $c[x + y * $j]);
                    palette.closest(color).2 as u32
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

pub fn remap<'a, const C: usize>(
    image: Image<&[f32], C>,
    palette: pal<'a, C>,
) -> out<'a, pal<'a, C>> {
    unsafe {
        IndexedImage::build(image.width(), image.height())
            .pal(palette)
            .buf_unchecked(
                image
                    .chunked()
                    .map(|x| palette.nearest(*x) as u32)
                    .collect(),
            )
    }
}

const BLUE: Image<[f32; 1024 * 1024], 1> = unsafe {
    Image::new(
        std::num::NonZero::new(1024).unwrap(),
        std::num::NonZero::new(1024).unwrap(),
        std::mem::transmute(*include_bytes!("../blue_1.f32")),
    )
};

// const Γ: f32 = 2.4;
const A: f32 = 12.92;

const U: f32 = 0.04045;
const C: f32 = 0.055;
fn srgb_to_lin(x: f32, γ: f32) -> f32 {
    // x.powf(1.0 / Γ)
    // https://wikimedia.org/api/rest_v1/media/math/render/svg/e401b31b97a8ddcf1de2b87b3606a278a645324e
    if x <= U {
        // x / A
        x * (1.0 / A)
    } else {
        // x + C / ⎞ ^ Γ
        // 1 + C   ⎠
        ((x + C) * (1.0 / (1.0 + C))).powf(γ)
    }
}

const V: f32 = 0.0031308;
fn lin_to_srgb(x: f32, γ: f32) -> f32 {
    // x.powf(Γ)
    if x <= V {
        A * x
    } else {
        // (1 + C)x¹⸍ᵞ - C
        (1.0 + C) * x.powf(1.0 / γ) - C
    }
}

pub fn encode<const C: usize, T: AsRef<[f32]>>(
    γ: f32, image: Image<T, C>
) -> Image<Box<[f32]>, C> {
    Image::build(image.width(), image.height()).buf(
        image
            .chunked()
            .flat_map(|x| x.map(|x| srgb_to_lin(x, γ)))
            .collect(),
    )
}

pub fn decode<const C: usize, T: AsRef<[f32]>>(
    γ: f32, image: Image<T, C>
) -> Image<Box<[f32]>, C> {
    Image::build(image.width(), image.height()).buf(
        image
            .chunked()
            .flat_map(|x| x.map(|x| lin_to_srgb(x, γ)))
            .collect(),
    )
}

pub fn blue<'a, const C: usize>(
    image: Image<&[f32], C>,
    palette: pal<'a, C>,
) -> out<'a, pal<'a, C>> {
    dither_with::<1024, C>(
        image,
        |((x, y), p)| unsafe {
            let [noise] = BLUE.pixel(x as u32, y as u32);
            palette.nearest(p.add(noise - 0.5)) as u32
        },
        palette,
    )
}

pub fn triangular<'a, const C: usize>(
    image: Image<&[f32], C>,
    palette: pal<'a, C>,
) -> out<'a, pal<'a, C>>
where
{
    // https://computergraphics.stackexchange.com/questions/5904/whats-a-proper-way-to-clamp-dither-noise/5952#5952
    fn triangle(x: f32, noise: f32) -> f32 {
        let noise = if x < (0.5 / 255.) || x > (254.5 / 255.) {
            noise - 0.5
        } else {
            if noise < 0.5 {
                (2.0 * noise).sqrt() - 1.0
            } else {
                1.0 - (2.0 - 2.0 * noise).sqrt()
            }
        };
        x + noise * 0.9
    }

    dither_with::<1024, C>(
        image,
        |((x, y), p)| unsafe {
            let [noise] = BLUE.pixel(x as u32, y as u32);
            palette.nearest(p.map(|x| triangle(x, noise))) as u32
        },
        palette,
    )
}
