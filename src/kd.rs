/// A data structure for fast nearest color lookups in a palette.
use atools::prelude::*;

pub struct KD {
    mid_point: [f32; 4],
    index: u32,
    normal: [f32; 4],
    left: Option<Box<KD>>,
    right: Option<Box<KD>>,
}

struct KDNearest {
    index: u32,
    distance: f32,
}

trait Dot {
    fn dot(self, other: Self) -> f32;
}

impl Dot for [f32; 4] {
    fn dot(self, other: Self) -> f32 {
        self.amul(other).sum()
    }
}

impl KD {
    pub fn new(colors: &[[f32; 4]]) -> Self {
        assert!(colors.len() < u32::MAX as usize);
        let mut x = (0..colors.len() as u32).collect::<Vec<_>>();

        Self::_new(&mut x, colors)
    }

    fn _new(mut indices: &mut [u32], colors: &[[f32; 4]]) -> KD {
        assert!(indices.len() > 0);

        let middle = indices.len() / 2;

        let mut sum = [0.; 4];
        let mut sum2 = [0.; 4];
        for i in indices.iter() {
            let c = colors[*i as usize];
            sum = sum.aadd(c);
            sum2 = sum2.aadd(c.amul(c));
        }
        let [r, g, b, a] = { sum2.asub(sum.amul(sum).mul(1.0 / indices.len() as f32)) };
        let normal = if r > g && r > b && r > a {
            (1.0).join([0.; 3])
        } else if g > b && g > a {
            [0.0, 1.0].couple([0.; 2])
        } else if b > a {
            [0.; 2].couple([1., 0.])
        } else {
            [0.; 3].join(1.)
        };
        indices.sort_by(|a, b| {
            colors[*a as usize]
                .dot(normal)
                .partial_cmp(&colors[*b as usize].dot(normal))
                .unwrap()
        });
        let i = indices.len() / 2;
        let left = if i > 0 {
            Some(Box::new(KD::_new(&mut indices[0..i], colors)))
        } else {
            None
        };
        let right = if i + 1 < indices.len() {
            Some(Box::new(KD::_new(&mut indices[(i + 1)..], colors)))
        } else {
            None
        };
        KD {
            mid_point: colors[indices[i as usize] as usize],
            index: indices[i as usize],
            normal: normal,
            left: left,
            right: right,
        }
    }

    fn _find_nearest(
        &self,
        needle: [f32; 4],
        mut limit: f32,
        ignore_index: u32,
    ) -> Option<KDNearest> {
        let mut result = None;

        let diff = needle.asub(self.mid_point);
        let distance = diff.dot(diff).sqrt();

        if distance < limit && self.index != ignore_index {
            limit = distance;
            result = Some(KDNearest {
                index: self.index,
                distance: distance,
            })
        }

        let dot = diff.dot(self.normal);
        if dot <= 0.0 {
            if let Some(ref left) = self.left {
                if let Some(nearest) = left._find_nearest(needle, limit, ignore_index) {
                    limit = nearest.distance;
                    result = Some(nearest);
                }
            }

            if -dot < limit {
                if let Some(ref right) = self.right {
                    if let Some(nearest) = right._find_nearest(needle, limit, ignore_index) {
                        result = Some(nearest);
                    }
                }
            }
        } else {
            if let Some(ref right) = self.right {
                if let Some(nearest) = right._find_nearest(needle, limit, ignore_index) {
                    limit = nearest.distance;
                    result = Some(nearest);
                }
            }

            if dot < limit {
                if let Some(ref left) = self.left {
                    if let Some(nearest) = left._find_nearest(needle, limit, ignore_index) {
                        result = Some(nearest);
                    }
                }
            }
        }

        result
    }

    pub fn find_nearest(&self, color: [f32; 4]) -> u32 {
        self._find_nearest(color, f32::MAX, u32::MAX)
            .map(|x| x.index)
            .unwrap_or(0)
    }
}

fn occludes(origin: [f32; 4], occluder: [f32; 4], target: [f32; 4]) -> bool {
    let dir = occluder.asub(origin);
    dir.dot(dir) * 0.5 <= (target.asub(origin)).dot(dir)
}
