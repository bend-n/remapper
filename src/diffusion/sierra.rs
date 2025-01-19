use super::*;

pub fn sierra<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    for (x, y) in image.serpent() {
        #[rustfmt::skip]
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| {
                move |x: [f32; 4]| {
                    x.aadd(error.amul([(f as f32 / 32.) * const { FAC as f32 * (1.0 / 255.) }; 4]))
                }
            };
            /*  * 5 3
            2 4 5 4 2
              2 3 2 */
            image.replace(x +             1, y, f(5));
            image.replace(x +             2, y, f(3));
            let y = y + 1;
            image.replace(x.wrapping_sub(2), y, f(2));
            image.replace(x.wrapping_sub(1), y, f(4));
            image.replace(x                , y, f(5));
            image.replace(x             + 1, y, f(4));
            image.replace(x             + 2, y, f(2));
            let y = y + 1;
            image.replace(x.wrapping_sub(1), y, f(2));
            image.replace(x                , y, f(3));
            image.replace(x             + 1, y, f(2));
        }
    }
    image
}

pub fn sierra_two<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    for (x, y) in image.serpent() {
        #[rustfmt::skip]
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| {
                move |x: [f32; 4]| {
                    x.aadd(error.amul([(f as f32 / 16.) * const { FAC as f32 * (1.0 / 255.) }; 4]))
                }
            };
            /*  * 4 3
            1 2 3 2 1 */
            image.replace(x +             1, y, f(4));
            image.replace(x +             2, y, f(3));
            let y = y + 1;
            image.replace(x.wrapping_sub(2), y, f(1));
            image.replace(x.wrapping_sub(1), y, f(2));
            image.replace(x                , y, f(3));
            image.replace(x             + 1, y, f(2));
            image.replace(x             + 2, y, f(1));
        }
    }
    image
}

pub fn sierra_lite<const FAC: u8>(
    image: Image<&[f32], 4>,
    palette: &[[f32; 4]],
) -> Image<Box<[f32]>, 4> {
    let kd = map(palette);
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    for (x, y) in image.serpent() {
        #[rustfmt::skip]
        unsafe {
            let p = image.pixel(x, y);
            let new = palette[kd.find_nearest(p) as usize];
            *image.pixel_mut(x, y) = new;
            let error = p.asub(new);
            let f = |f| {
                move |x: [f32; 4]| {
                    x.aadd(error.amul([(f as f32 / 4.) * const { FAC as f32 * (1.0 / 255.) }; 4]))
                }
            };
            #[allow(warnings)]
            /** 2
            1 1 */
            image.replace(x +             1, y, f(2));
            let y = y + 1;
            image.replace(x.wrapping_sub(1), y, f(1));
            image.replace(x                , y, f(1));
        }
    }
    image
}
