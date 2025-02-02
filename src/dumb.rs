use atools::prelude::*;
use lower::algebraic::math;
pub trait Closest<const N: usize> {
    fn closest(&self, color: [f32; N]) -> (f32, [f32; N], u32);
    fn best(&self, color: [f32; N]) -> [f32; N] {
        self.closest(color).1
    }
    fn nearest(&self, color: [f32; N]) -> u32 {
        self.closest(color).2
    }
    fn space(&self) -> f32;
}

#[inline(always)]
fn euclidean_distance<const N: usize>(f: [f32; N], with: [f32; N]) -> f32 {
    math! {
        f.asub(with)
            .map(|x| x*x)
            .into_iter()
            .fold(0.0, |acc, x| acc + x)
    }
}

#[inline(always)]
fn minwby<T: Copy, U: PartialOrd>(max: T, x: impl Iterator<Item = T>, extractor: fn(T) -> U) -> T {
    x.fold(max, |acc, x| {
        if extractor(acc) > extractor(x) {
            x
        } else {
            acc
        }
    })
}

impl<'a, const N: usize> Closest<N> for super::pal<'a, N> {
    /// o(nn)
    fn closest(&self, color: [f32; N]) -> (f32, [f32; N], u32) {
        minwby(
            (f32::MAX, [0.0; N], 0),
            (0..)
                .zip(&**self)
                .map(|(i, &x)| (euclidean_distance(x, color), x, i)),
            |x| x.0,
        )
    }

    /// o(nn)
    #[cold]
    fn space(&self) -> f32 {
        math! {
        self.iter()
            .enumerate()
            .map(|(i, &x)| {
                minwby(f32::MAX, self.iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &y)| euclidean_distance(y, x)),
                std::convert::identity).sqrt()
            })
            .fold(0.0, |x, y| x + y)
            / self.len() as f32
        }
    }
}
