//! https://www.compuphase.com/riemer.htm
use super::*;

#[test]
fn x() {
    let mut q = Ring::new([0.0, 2.0, 5.0]);
    dbg!(q.iter().collect::<Vec<_>>());
    assert_eq!(q.pop_front_push_back(6.0), 0.0);
    dbg!(q.iter().collect::<Vec<_>>());
    assert_eq!(q.pop_front_push_back(3.0), 2.0);
    dbg!(q.iter().collect::<Vec<_>>());
    assert_eq!(q.pop_front_push_back(4.0), 5.0);
    dbg!(q.iter().collect::<Vec<_>>());
    assert_eq!(q.pop_front_push_back(7.0), 6.0);
    dbg!(q.iter().collect::<Vec<_>>());
}

pub struct Ring<T, const N: usize> {
    arr: [T; N],
    front: u8,
}
impl<T, const N: usize> Ring<T, N> {
    pub fn new(contents: [T; N]) -> Self {
        Ring {
            arr: contents,
            front: 0,
        }
    }
    pub fn pop_front_push_back(&mut self, push_back: T) -> T {
        unsafe {
            let e = std::mem::replace(self.arr.get_unchecked_mut(self.front as usize), push_back);
            self.front += 1;
            self.front %= N as u8;
            e
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.arr.iter().cycle().skip(self.front as _).take(N as _)
    }
}

pub fn riemerasma(image: Image<&[f32], 4>, palette: pal<'_, 4>) -> Image<Box<[f32]>, 4> {
    let mut image =
        Image::build(image.width(), image.height()).buf(image.buffer().to_vec().into_boxed_slice());
    #[rustfmt::skip]
    const WEIGH: [f32; 16] = [0.0625, 0.07518906, 0.09045432, 0.10881881, 0.13091174, 0.15749009, 0.18946451, 0.22793055, 0.27420613, 0.32987684, 0.39685008, 0.47742057, 0.57434887, 0.69095606, 0.8312374, 1.0];
    let mut errors = Ring::<[f32; 4], 16>::new([[0.; 4]; 16]);
    let mut level = image.width().max(image.height()).ilog2();
    if (1 << level) < image.width().max(image.height()) {
        level += 1;
    }
    // static mut visualization: Image<[u8; 256 * 256], 1> = fimg::make!(1 channels 256 x 256);
    // static mut stage: u8 = 0;
    enum Dir {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }
    use Dir::*;
    hl(
        level,
        UP,
        &mut (&mut (0, 0), &mut errors, &(), palette, &mut image),
    );
    fn hl(
        level: u32,
        dir: Dir,
        p: &mut (
            &mut (i32, i32),
            &mut Ring<[f32; 4], 16>,
            &(),
            pal<4>,
            &mut Image<Box<[f32]>, 4>,
        ),
    ) {
        macro_rules! mv {
            ($dir: expr) => {{
                unsafe {
                    if p.0 .0 >= 0
                        && p.0 .0 < p.4.width() as i32
                        && p.0 .1 >= 0
                        && p.0 .1 < p.4.height() as i32
                    {
                        let error =
                            p.1.iter()
                                .zip(WEIGH)
                                .map(|(&a, b)| a.mul(b))
                                .fold([0.; 4], |acc, x| acc.aadd(x))
                                .div(WEIGH.len() as f32);
                        let (x, y) = *p.0;
                        let (x, y) = (x as u32, y as u32);
                        // visualization.set_pixel(x, y, [stage]);
                        // stage += 1;
                        let px = p.4.pixel(x, y).aadd(error);
                        let np = p.3.best(px);
                        p.1.pop_front_push_back(px.asub(np));
                        *p.4.pixel_mut(x, y) = np;
                    }
                    match $dir {
                        LEFT => p.0 .0 -= 1,
                        RIGHT => p.0 .0 += 1,
                        UP => p.0 .1 -= 1,
                        DOWN => p.0 .1 += 1,
                    }
                }
            }};
        }
        macro_rules! dir {
            (^) => {
                UP
            };
            (>) => {
                RIGHT
            };
            (<) => {
                LEFT
            };
            (v) => {
                DOWN
            };
        }
        macro_rules! pattern {
            ($($x:tt)+) => {{
                $(mv!(dir!($x));)+
            }};
        }
        macro_rules! hilbert {
            ($a1:tt $b1:tt $a2:tt $b2:tt $a3:tt $b3:tt $b4:tt) => {{
                hl(level - 1, dir!($a1), p);
                mv!(dir!($b1));

                hl(level - 1, dir!($a2), p);
                mv!(dir!($b2));

                hl(level - 1, dir!($a3), p);
                mv!(dir!($b3));

                hl(level - 1, dir!($b4), p);
            }};
        }
        if level == 1 {
            match dir {
                LEFT => pattern!(>v<),
                RIGHT => pattern!(<^>),
                UP => pattern!(v>^),
                DOWN => pattern!(^<v),
            }
        } else {
            match dir {
                LEFT => hilbert!(^> <v < < v),
                RIGHT => hilbert!(v< >^ > > ^),
                UP => hilbert!(<v ^> ^^ >),
                DOWN => hilbert!(>^ v< v v <),
            }
        }
    }
    // unsafe { visualization.as_ref().show() };
    image
}
