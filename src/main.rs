#![feature(slice_as_chunks, generic_const_exprs)]
use atools::prelude::*;
use exoquant::SimpleColorSpace;
use rand::{Rng, RngCore};

fn main() {
    reemap();
    // eomap();
    // let mut rng = rand::thread_rng();
    // let pal = std::iter::repeat_with(|| {
    //     let a: [f32; 3] = std::array::from_fn(|_| rng.next_u64() as f32 / u64::MAX as f32);
    //     a.join(1.)
    // })
    // .take(200)
    // .collect::<Vec<_>>();
}

fn reemap() {
    // let mut rng = rand::thread_rng();
    // let pal = std::iter::repeat_with(|| {
    //     let a: [f32; 3] = std::array::from_fn(|_| rng.next_u64() as f32 / u64::MAX as f32);
    //     a.join(1.)
    // })
    // .take(5124)
    // .collect::<Vec<_>>();
    let pal = fimg::Image::<Box<[f32]>, 4>::from(fimg::Image::open("../endesga.png").as_ref());
    let pal = pal.flatten();

    /*let pal = [
        ]
        .chunked::<3>()
        .map(|x| x.join(255).map(|x| x as f32 * (1.0 / 255.0)));
    */
    // println!("{pal:?}");

    fimg::Image::<Box<[u8]>, 4>::from(
        remapper::remap(
            fimg::Image::<Box<[f32]>, 4>::from(
                fimg::Image::<Vec<u8>, 4>::open("plane_4096_2048.png").as_ref(),
            )
            .as_ref(),
            &pal,
        )
        .as_ref(),
    )
    .save("yeee.png");
}

fn eomap() {
    let x = fimg::Image::<Vec<u8>, 4>::open("../fimg/tdata/cat.png");
    let pal = fimg::Image::open("../endesga.png");
    let pal = pal.flatten();
    let res = exoquant::Remapper::new(
        &pal.iter()
            .map(|&[r, g, b, a]| exoquant::Color::new(r, g, b, a))
            .collect::<Vec<_>>(),
        &SimpleColorSpace::default(),
        &exoquant::ditherer::Ordered,
    )
    .remap(
        &x.chunked()
            .map(|&[r, g, b, a]| exoquant::Color::new(r, g, b, a))
            .collect::<Vec<_>>(),
        x.width() as usize,
    );
    let (width, height) = (x.width() as usize, x.height() as usize);
    let pixels = (0..width)
        .flat_map(|x_| (0..height).map(move |y| (x_, y)))
        .filter_map(move |(x_, y_)| match res[(height - y_ - 1) * width + x_] {
            0 => None,
            x => Some(((x_, y_), x)),
        });
    let mut preview = fimg::Image::build(x.width(), x.height()).fill([0; 3].join(255));
    for ((x, y), i) in pixels {
        unsafe { preview.set_pixel(x as _, (height - y - 1) as _, pal[i as usize]) };
    }
    preview.save("eoquan.png");
}