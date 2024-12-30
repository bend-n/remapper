//! # Ordered dithering.
//! The way this works is by adding a constant texture to the image, and then quantizing that.
use super::*;
pub fn remap_bayer_2x2(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<2>(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_2X2[x + y * 2]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_4x4(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<4>(image, |((x, y), &p)| {
        let color = p.add(r * BAYER_4X4[x + y * 4]);
        palette[kd.find_nearest(color) as usize]
    })
}

pub fn remap_bayer_8x8(image: Image<&[f32], 4>, palette: &[[f32; 4]]) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let r = kd.space(palette);
    dither_with::<8>(image, |((x, y), &p)| {
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
const BLUE: Image<[f32; 1024 * 1024 * 3], 3> = unsafe {
    Image::new(
        std::num::NonZero::new(1024).unwrap(),
        std::num::NonZero::new(1024).unwrap(),
        std::mem::transmute(*include_bytes!("../blue.f32")),
    )
};
// todo: figure this out? seems off.
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
