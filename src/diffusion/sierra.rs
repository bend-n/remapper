use super::*;
#[no_mangle]
fn seeerad<'p>(x: Image<Box<[f32]>, 1>, palette: pal<'p, 1>) -> out<'p, pal<'p, 1>>{
    sierra::<255,1>(x, palette)
}


pub fn sierra<'p, const FAC: u8, const N: usize>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'p, N>,
) -> out<'p, pal<'p, N>> {
    let out = out::build(image.width(), image.height()).pal(palette);
    let fac = const { FAC as f32 / 255.0 };
    #[rustfmt::skip]
    let i = image.serpent().map(|c @ (x, y)| unsafe { 
        let p = image.pixel(x, y);
        let (_, new, i) = palette.closest(p);
        *image.pixel_mut(x, y) = new;
        let error = p.asub(new);
        let f = |f| {
            move |x: [f32; N]| { lower::algebraic::math! {
                x.zip(error).map(|(x, error)| error * (f  as f32 / 32.) * fac + x)
            } }
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
        (c, i)
    });
    unsafe { out.from_iter(i) }
}

pub fn sierra_two<'p, const FAC: u8, const N: usize>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'p, N>,
) -> out<'p, pal<'p, N>> {
    let out = out::build(image.width(), image.height()).pal(palette);
    #[rustfmt::skip]
    let i = image.serpent().map(|c@(x, y)| unsafe {
        let p = image.pixel(x, y);
        let (_, new, i) = palette.closest(p);
        *image.pixel_mut(x, y) = new;
        let error = p.asub(new);
        let f = |f| {
            move |x: [f32; N]| {
                x.aadd(error.amul([(f as f32 / 16.) * const { FAC as f32 * (1.0 / 255.) }; N]))
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
        (c, i)
    });
    unsafe { out.from_iter(i) }
}

pub fn sierra_lite<'p, const FAC: u8, const N: usize>(
    mut image: Image<Box<[f32]>, N>,
    palette: pal<'p, N>,
) -> out<'p, pal<'p, N>> {
    let out = out::build(image.width(), image.height()).pal(palette);
    #[rustfmt::skip]
    let i = image.serpent().map(|c@(x, y)| unsafe {
        let p = image.pixel(x, y);
        let (_, new, i) = palette.closest(p);
        *image.pixel_mut(x, y) = new;
        let error = p.asub(new);
        let f = |f| {
            move |x: [f32; N]| {
                x.aadd(error.amul([(f as f32 / 4.) * const { FAC as f32 * (1.0 / 255.) }; N]))                
            }
        };
        
        #[allow(warnings)]
        /** 2
        1 1 */
        image.replace(x +             1, y, f(2));
        let y = y + 1;
        image.replace(x.wrapping_sub(1), y, f(1));
        image.replace(x                , y, f(1));
        (c, i)
    });
    unsafe { out.from_iter(i) }
}
