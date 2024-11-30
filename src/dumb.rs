use atools::prelude::*;
pub trait Closest {
    fn closest(&self, color: [f32; 4]) -> (f32, [f32; 4], usize);
}

fn euclidean_distance(f: [f32; 4], with: [f32; 4]) -> f32 {
    f.asub(with).map(|x| x * x).sum()
}

impl Closest for &[[f32; 4]] {
    fn closest(&self, color: [f32; 4]) -> (f32, [f32; 4], usize) {
        self.iter()
            .enumerate()
            .map(|(i, x)| (euclidean_distance(*x, color), x, i))
            .min_by(|x, y| x.0.total_cmp(&y.0))
            .map(|(d, x, i)| (d, *x, i))
            .unwrap()
        // let mut best = (euclidean_distance(self[0], color), self[0], 0);
        // for (&c, i) in self[1..].iter().zip(1..) {
        //     let d = euclidean_distance(c, color);
        //     if d < best.0 {
        //         best = (d, c, i);
        //     }
        // }
        // best
    }
}
