use std::ops::Range;

use crate::glam::IVec2;

use crate::{circle_ivec2::CircleIVec2, RectangleIVec2};

use super::common::Scanline;

#[derive(Clone, Debug)]
pub struct CirclePoints {
    scanlines: Scanlines,
    current_scanline: Scanline,
}

impl CirclePoints {
    pub(crate) fn new(circle: &CircleIVec2, outline: bool) -> Self {
        Self {
            scanlines: Scanlines::new(circle, outline),
            current_scanline: Scanline::new_empty(0),
        }
    }
}

impl Iterator for CirclePoints {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_scanline.next().or_else(|| {
            self.current_scanline = self.scanlines.next()?;
            self.current_scanline.next()
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Scanlines {
    outline: bool,
    rows: Range<i32>,
    columns: Range<i32>,
    center_2x: IVec2,
    threshold: i32,
    limits: Option<RectangleIVec2>,
}

impl Scanlines {
    pub fn new(circle: &CircleIVec2, outline: bool) -> Self {
        let bounding_box = circle.bounding_box();
        let limits = circle.limits;
        Self {
            outline,
            rows: bounding_box.rows_limited(&limits),
            columns: bounding_box.columns(),
            center_2x: circle.center_2x(),
            threshold: circle.threshold(),
            limits,
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.rows.next()?;



        self.columns
            .clone()
            // find first pixel that is inside the threshold
            .find(|x| {
                // if *x < self.limits.tl.x || *x >= self.limits.br.x {
                //     return false;
                // }
                let delta = IVec2::new(*x, y) * 2 - self.center_2x;
                delta.length_squared() < self.threshold
            })
            // shorten the scanline by right side of the same amount as the left side
            .map(|x| { 
                if let Some(limits) = self.limits {
                    let start_x = x.max(limits.tl.x);
                    let last_x = self.columns.end - (x - self.columns.start);
                    Scanline::new(y, start_x..last_x.min(limits.br.x), self.outline) 
                } else {
                    Scanline::new(y, x..self.columns.end - (x - self.columns.start), self.outline) 
                }

            }) 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() {
        let circle = CircleIVec2::new(IVec2::new(10, 10), 3);
        let points: Vec<IVec2> = circle.pixel_iter(false).collect();

        // assert_eq!(points.len(), 32);

        assert_eq!(
            points,
            vec![
                IVec2::new(8, 7),
                IVec2::new(9, 7),
                IVec2::new(10, 7),
                IVec2::new(11, 7),
                IVec2::new(7, 8),
                IVec2::new(8, 8),
                IVec2::new(9, 8),
                IVec2::new(10, 8),
                IVec2::new(11, 8),
                IVec2::new(12, 8),
                IVec2::new(7, 9),
                IVec2::new(8, 9),
                IVec2::new(9, 9),
                IVec2::new(10, 9),
                IVec2::new(11, 9),
                IVec2::new(12, 9),
                IVec2::new(7, 10),
                IVec2::new(8, 10),
                IVec2::new(9, 10),
                IVec2::new(10, 10),
                IVec2::new(11, 10),
                IVec2::new(12, 10),
                IVec2::new(7, 11),
                IVec2::new(8, 11),
                IVec2::new(9, 11),
                IVec2::new(10, 11),
                IVec2::new(11, 11),
                IVec2::new(12, 11),
                IVec2::new(8, 12),
                IVec2::new(9, 12),
                IVec2::new(10, 12),
                IVec2::new(11, 12)
            ]
        );
    }

    // #[test]
    // fn test_iter_with_limits() {
    //     let circle = CircleIVec2::new_with_limits(
    //         IVec2::new(10, 10), 
    //         3, 
    //         RectangleIVec2::new(IVec2::new(10, 10), IVec2::new(20, 20))
    //     );
    //     let points: Vec<IVec2> = circle.pixel_iter(false).collect();

    //     assert_eq!(points.len(), 8);

    //     assert_eq!(
    //         points,
    //         vec![
    //             IVec2::new(10, 10),
    //             IVec2::new(11, 10),
    //             IVec2::new(10, 11),
    //             IVec2::new(11, 11),
    //             IVec2::new(10, 12),
    //             IVec2::new(11, 12),
    //             IVec2::new(10, 13),
    //             IVec2::new(11, 13),
    //         ]
    //     );
    // }
}
