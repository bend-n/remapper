#![feature(slice_as_chunks, generic_arg_infer, iter_chain)]
use fimg::{DynImage, Image};
use std::time::Instant;

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
    // println!("{:?}", ordered::BAYER_8x8Q);
    // let mut rng = rand::thread_rng();
    // let pal = std::iter::repeat_with(|| {
    //     let a: [f32; 3] = std::array::from_fn(|_| rng.next_u64() as f32 / u64::MAX as f32);
    //     a.join(1.)
    // })
    // .take(5124)
    // .collect::<Vec<_>>();
    // let mut new = Image::<Vec<u8>, 1>::build(256, 100).alloc();
    // for pixels in unsafe { new.buffer_mut().chunks_mut(256) } {
    //     pixels.copy_from_slice(&(0..=u8::MAX).collect::<Vec<_>>());
    // }
    // new.save("gradient.png");
    let pal = Image::<Box<[f32]>, 4>::from(Image::open("tdata/optimal.png").as_ref());
    let pal = pal.flatten();

    // let pal = &[
    //     [0., 0., 0., 1.],
    //     [0.25, 0.25, 0.25, 1.],
    //     [0.5, 0.5, 0.5, 1.],
    //     [0.75, 0.75, 0.75, 1.],
    //     [1.; 4],
    // ][..];
    // let pal = &[[0.], [1.]][..];
    // let pal = &[[0.], [0.25], [0.5], [0.75], [1.]][..];
    // let pal = &[0.1, 0.2, 0.3, 0.4, 0.5, 0.7, 0.9, 1.0].map(|x| [x])[..];

    /*let pal = [
        ]
        .chunked::<3>()
        .map(|x| x.join(255).map(|x| x as f32 * (1.0 / 255.0)));
    */
    // println!("{pal:?}");
    // dbg!(pal.space());
    let i = DynImage::open("../fimg/tdata/cat.png").to_rgba();
    // let pal = [[0.], [1.]];
    // let mut pal = exoquant::generate_palette(
    //     &i.chunked()
    //         .map(|&[r, g, b, a]| exoquant::Color::new(r, g, b, a))
    //         .collect::<exoquant::Histogram>(),
    //     &exoquant::SimpleColorSpace::default(),
    //     &exoquant::optimizer::KMeans,
    //     64,
    // )
    // .into_iter()
    // .map(|exoquant::Color { r, g, b, a }| [r, g, b, a].map(|x| x as f32 * (1.0 / 255.0)))
    // .collect::<Vec<_>>();
    // pal.sort_by(|a, b| {
    //     let lum = |[r, g, b, _]: [f32; 4]| 0.2126 * r + 0.7152 * g + 0.0722 * b;
    //     lum(*a).total_cmp(&lum(*b))
    // });
    // Image::<_, 4>::build(8, 8)
    //     .buf(pal.as_slice().as_flattened())
    //     .to_u8()
    //     .save("tdata/optimal.png");
    // i.save("gamma/gray.png");
    let i = i.to_f32();
    // decode(2.2, encode(1.0, i.as_ref()))
    //     .to_u8()
    //     .save("gamma/1_0.png");
    // decode(2.2, encode(2.0, i.as_ref()))
    //     .to_u8()
    //     .save("gamma/2_0.png");
    // decode(2.2, encode(2.2, i.as_ref()))
    //     .to_u8()
    //     .save("gamma/2_2.png");
    // decode(2.2, encode(2.4, i.as_ref()))
    //     .to_u8()
    //     .save("gamma/2_4.png");
    let now = Instant::now();
    let x = remapper::diffusion::sierra::sierra::<255, 4>(
        // fimg::Image::<&[u8], 4>::make::<256, 256>().as_ref(),
        i, &pal,
    )
    .to()
    .to_u8();
    dbg!(now.elapsed());
    x.save("yeee.png");
    // .show()
    // .save("yeee.png");
}
