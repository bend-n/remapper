use atools::prelude::*;

pub struct KD {
    median: [f32; 4],
    normal: [f32; 4],
    index: u32,
    left: Option<Box<KD>>,
    right: Option<Box<KD>>,
    depth: u32,
}

impl std::fmt::Debug for KD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {:?}", self.index, self.median)?;
        let d = self.depth as usize;
        if let Some(ref left) = self.left {
            write!(f, "\n{}  ⬅️  {left:?}", " ".repeat(d))?;
        }
        if let Some(ref right) = self.right {
            write!(f, "\n{}  ➡️  {right:?}", " ".repeat(d))?;
        }

        Ok(())
    }
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
        let colors = colors.iter().copied().zip(0u32..).collect::<Vec<_>>();
        Self::_new(&mut { colors }, 0).unwrap()
    }

    fn _new(colors: &mut [([f32; 4], u32)], depth: u32) -> Option<KD> {
        if colors.len() == 0 {
            return None;
        };

        let middle = colors.len() / 2;

        let mut sum = [0.; 4];
        let mut sum2 = [0.; 4];
        for &(c, _) in colors.iter() {
            sum = sum.aadd(c);
            sum2 = sum2.aadd(c.amul(c));
        }
        let [r, g, b, a] = { sum2.asub(sum.amul(sum).mul(1.0 / colors.len() as f32)) };

        let normal = [
            (r > g && r > b && r > a) as u8 as f32,
            (g > b && g > a) as u8 as f32,
            (b > a) as u8 as f32,
            1.,
        ];
        // colors.sort_by(|(a, _), (b, _)| );
        let (before, median, after) = colors.select_nth_unstable_by(middle, |(a, _), (b, _)| {
            a.dot(normal).partial_cmp(&b.dot(normal)).unwrap()
        });
        Some(KD {
            median: median.0,
            index: median.1,
            left: KD::_new(before, depth + 1).map(Box::new),
            right: KD::_new(after, depth + 1).map(Box::new),
            normal,
            depth,
        })
    }

    fn _find_nearest(&self, needle: [f32; 4], mut limit: f32) -> Option<KDNearest> {
        let mut result = None;

        let diff = needle.asub(self.median);
        let distance = diff.dot(diff).sqrt();

        if distance < limit {
            limit = distance;
            result = Some(KDNearest {
                index: self.index,
                distance,
            })
        }

        let dot = diff.dot(self.normal);
        if dot <= 0.0 {
            if let Some(ref left) = self.left
                && let Some(nearest) = left._find_nearest(needle, limit)
            {
                limit = nearest.distance;
                result = Some(nearest);
            }

            if -dot < limit {
                if let Some(ref right) = self.right
                    && let Some(nearest) = right._find_nearest(needle, limit)
                {
                    result = Some(nearest);
                }
            }
        } else {
            if let Some(ref right) = self.right
                && let Some(nearest) = right._find_nearest(needle, limit)
            {
                limit = nearest.distance;
                result = Some(nearest);
            }

            if dot < limit {
                if let Some(ref left) = self.left
                    && let Some(nearest) = left._find_nearest(needle, limit)
                {
                    result = Some(nearest);
                }
            }
        }

        result
    }

    fn _find_nearest_excepting(
        &self,
        needle: [f32; 4],
        mut limit: f32,
        ignore_index: u32,
    ) -> Option<KDNearest> {
        let mut result = None;

        let diff = needle.asub(self.median);
        let distance = diff.dot(diff).sqrt();

        if distance < limit && self.index != ignore_index {
            limit = distance;
            result = Some(KDNearest {
                index: self.index,
                distance,
            })
        }

        let dot = diff.dot(self.normal);
        if dot <= 0.0 {
            if let Some(ref left) = self.left
                && let Some(nearest) = left._find_nearest_excepting(needle, limit, ignore_index)
            {
                limit = nearest.distance;
                result = Some(nearest);
            }

            if -dot < limit {
                if let Some(ref right) = self.right
                    && let Some(nearest) =
                        right._find_nearest_excepting(needle, limit, ignore_index)
                {
                    result = Some(nearest);
                }
            }
        } else {
            if let Some(ref right) = self.right
                && let Some(nearest) = right._find_nearest_excepting(needle, limit, ignore_index)
            {
                limit = nearest.distance;
                result = Some(nearest);
            }

            if dot < limit {
                if let Some(ref left) = self.left
                    && let Some(nearest) = left._find_nearest_excepting(needle, limit, ignore_index)
                {
                    result = Some(nearest);
                }
            }
        }

        result
    }

    pub fn find_nearest(&self, color: [f32; 4]) -> u32 {
        self._find_nearest(color, f32::MAX)
            .map(|x| x.index)
            .unwrap_or(0)
    }

    pub fn space(&self, colors: &[[f32; 4]]) -> f32 {
        let mut i = colors
            .iter()
            .zip(0..)
            .map(|(c, i)| {
                self._find_nearest_excepting(*c, f32::MAX, i)
                    .unwrap()
                    .distance
            })
            .array_chunks::<256>();

        let (c, sum) = i.by_ref().fold((0.0, 0.0), |(c, sum), chunk| {
            let y = sum_block(chunk) - c;
            let t = sum + y;
            ((t - sum) - y, t)
        });
        (sum + (i.into_remainder().map(sum_block).unwrap_or(0.) - c)) / colors.len() as f32
    }
}

use std::intrinsics::fadd_algebraic;
fn sum_block(arr: impl IntoIterator<Item = f32>) -> f32 {
    arr.into_iter().fold(0.0, |x, y| fadd_algebraic(x, y))
}
