use std::ops::Range;

use glam::IVec2;

use crate::ivec2;


/// Iterator over all IVec2s inside the rectangle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RectanglePixels {
    outline: bool,
    x: Range<i32>,
    y: Range<i32>,
    x_start: i32,
    y_start: i32,
}

impl RectanglePixels {
    pub(crate) fn new(rectangle: &ivec2::Rectangle, outline: bool) -> Self {
        // eprintln!("RectanglePixels rect: {:?}, outline: {:?}", rectangle, outline);
        // Return `Self::empty` for all zero sized rectangles.
        // The iterator would behave correctly without this check, but would loop unnecessarily for
        // rectangles with zero width.
        if rectangle.is_zero_sized() {
            return Self::empty();
        }

        let x = rectangle.columns();
        let y = rectangle.rows();
        let x_start = x.start;
        let y_start = y.start;

        Self { x, y, x_start, y_start, outline }
    }

    /// Create a points iterator that returns no items.
    pub const fn empty() -> Self {
        Self {
            x: 0..0,
            y: 0..0,
            x_start: 0,
            y_start: 0,
            outline: false,
        }
    }
}

impl Iterator for RectanglePixels {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while !self.y.is_empty() {
            if let Some(x) = self.x.next() {
                if !self.outline || self.y.start == self.y_start || self.y.start == self.y.end - 1 {
                    return Some(IVec2::new(x, self.y.start));
                } else if self.outline {
                    if x == self.x_start || x == self.x.end - 1 {
                        if x == self.x_start {
                            self.x.start = self.x.end - 1;
                        }
                        return Some(IVec2::new(x, self.y.start));
                    } 
                }
            }
            // eprintln!("no x: {}", self.y.start);

            self.y.next();
            self.x.start = self.x_start;
        }

        None
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rectangle_pixels() {
        let rect = ivec2::Rectangle::new(IVec2::new(0, 0), IVec2::new(2, 2));
        let pixels: Vec<IVec2> = RectanglePixels::new(&rect, false).collect();
        assert_eq!(pixels, vec![
            IVec2::new(0, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ]);
    }

    // RectanglePixels rect: ivec2::Rectangle { tl: IVec2(125, 34), br: IVec2(129, 39) }, outline: true
    #[test]
    fn outline_y_start_not_zero() {
        let rect = ivec2::Rectangle::new(IVec2::new(0, 34), IVec2::new(4, 39));
        let pixels: Vec<IVec2> = RectanglePixels::new(&rect, true).collect();
        assert_eq!(pixels, vec![
            IVec2::new(0, 34),
            IVec2::new(1, 34),
            IVec2::new(2, 34),
            IVec2::new(3, 34),

            IVec2::new(0, 35),
            IVec2::new(3, 35),

            IVec2::new(0, 36),
            IVec2::new(3, 36),

            IVec2::new(0, 37),
            IVec2::new(3, 37),

            IVec2::new(0, 38),
            IVec2::new(1, 38),
            IVec2::new(2, 38),
            IVec2::new(3, 38),


        ]);
    }

    #[test]
    fn outline_x_start_not_zero() {
        let rect = ivec2::Rectangle::new(IVec2::new(10, 0), IVec2::new(14, 4));
        let pixels: Vec<IVec2> = RectanglePixels::new(&rect, true).collect();
        assert_eq!(pixels, vec![
            IVec2::new(10, 0),
            IVec2::new(11, 0),
            IVec2::new(12, 0),
            IVec2::new(13, 0),

            IVec2::new(10, 1),
            IVec2::new(13, 1),

            IVec2::new(10, 2),
            IVec2::new(13, 2),

            IVec2::new(10, 3),
            IVec2::new(11, 3),
            IVec2::new(12, 3),
            IVec2::new(13, 3),


        ]);
    }


    #[test]
    fn rectangle_outline_pixels() {
        let rect = ivec2::Rectangle::new(IVec2::new(0, 0), IVec2::new(4, 4));
        let pixels: Vec<IVec2> = RectanglePixels::new(&rect, true).collect();
        // // top row
        // assert!(pixels.contains(&IVec2::new(0, 0)));
        // assert!(pixels.contains(&IVec2::new(1, 0)));
        // assert!(pixels.contains(&IVec2::new(2, 0)));
        // assert!(pixels.contains(&IVec2::new(3, 0)));

        // // second row
        // assert!(pixels.contains(&IVec2::new(0, 1)));
        // assert!(pixels.contains(&IVec2::new(3, 1)));

        // // third row
        // assert!(pixels.contains(&IVec2::new(0, 2)));
        // assert!(pixels.contains(&IVec2::new(3, 2)));

        // // fourth row
        // assert!(pixels.contains(&IVec2::new(0, 3)));
        // assert!(pixels.contains(&IVec2::new(1, 3)));
        // assert!(pixels.contains(&IVec2::new(2, 3)));
        // assert!(pixels.contains(&IVec2::new(3, 3)));

        // assert_eq!(pixels.len(), 12);

        assert_eq!(pixels, vec![
            IVec2::new(0, 0),
            IVec2::new(1, 0),
            IVec2::new(2, 0),
            IVec2::new(3, 0),
            IVec2::new(0, 1),
            IVec2::new(3, 1),
            IVec2::new(0, 2),
            IVec2::new(3, 2),
            IVec2::new(0, 3),
            IVec2::new(1, 3),
            IVec2::new(2, 3),
            IVec2::new(3, 3),
        ]);

        // top row
        assert_eq!(pixels[0..4], vec![
            IVec2::new(0, 0),
            IVec2::new(1, 0),
            IVec2::new(2, 0),
            IVec2::new(3, 0),
        ]);

        // second row
        assert_eq!(pixels[4..6], vec![
            IVec2::new(0, 1),
            IVec2::new(3, 1),
        ]);

        // third row
        assert_eq!(pixels[6..8], vec![
            IVec2::new(0, 2),
            IVec2::new(3, 2),
        ]);

        // fourth row
        assert_eq!(pixels[8..12], vec![
            IVec2::new(0, 3),
            IVec2::new(1, 3),
            IVec2::new(2, 3),
            IVec2::new(3, 3),
        ]);

    }

//     #[test]
//     fn rectangle_outline_pixels_vs() {
//         let rect = ivec2::Rectangle::new(IVec2::new(0, 0), IVec2::new(4, 4));
//         let pixels: Vec<IVec2> = RectangleOutlinePixels::new(&rect).collect();
//         let pixels2: Vec<IVec2> = RectanglePixels::new(&rect, true).collect();
//         // top row
//         assert!(pixels.contains(&IVec2::new(0, 0)));
//         assert!(pixels.contains(&IVec2::new(1, 0)));
//         assert!(pixels.contains(&IVec2::new(2, 0)));
//         assert!(pixels.contains(&IVec2::new(3, 0)));

//         // second row
//         assert!(pixels.contains(&IVec2::new(0, 1)));
//         assert!(pixels.contains(&IVec2::new(3, 1)));

//         // third row
//         assert!(pixels.contains(&IVec2::new(0, 2)));
//         assert!(pixels.contains(&IVec2::new(3, 2)));

//         // fourth row
//         assert!(pixels.contains(&IVec2::new(0, 3)));
//         assert!(pixels.contains(&IVec2::new(1, 3)));
//         assert!(pixels.contains(&IVec2::new(2, 3)));
//         assert!(pixels.contains(&IVec2::new(3, 3)));

//         assert_eq!(pixels.len(), 12);

//     }
}
