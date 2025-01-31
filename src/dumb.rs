use atools::prelude::*;
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

fn euclidean_distance<const N: usize>(f: [f32; N], with: [f32; N]) -> f32 {
    f.asub(with)
        .map(|x| std::intrinsics::fmul_algebraic(x, x))
        .sum()
}

#[no_mangle]
fn closeer(x: [f32; 4], p: &[[f32; 4]]) -> [f32; 4] {
    p.best(x)
}

impl<const N: usize> Closest<N> for &[[f32; N]] {
    /// o(nn)
    #[inline]
    fn closest(&self, color: [f32; N]) -> (f32, [f32; N], u32) {
        (0..)
            .zip(*self)
            .map(|(i, &x)| (euclidean_distance(x, color), x, i))
            .min_by(|x, y| x.0.total_cmp(&y.0))
            .unwrap()
    }

    /// o(nn)
    fn space(&self) -> f32 {
        self.iter()
            .enumerate()
            .map(|(i, &x)| {
                self.iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &y)| euclidean_distance(y, x))
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap()
                    .sqrt()
            })
            .fold(0.0, |x, y| std::intrinsics::fadd_algebraic(x, y))
            / self.len() as f32
    }
}
