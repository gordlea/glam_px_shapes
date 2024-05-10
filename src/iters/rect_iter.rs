use std::ops::Range;

use crate::glam::IVec2;

use crate::rectangle::rectangle_ivec2::RectangleIVec2;

use super::common::Scanline;


/// Iterator over all IVec2s inside the rectangle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RectanglePixels {
    x: Range<i32>,
    y: Range<i32>,
    x_start: i32,
}

impl RectanglePixels {
    pub(crate) fn new(rectangle: &RectangleIVec2) -> Self {
        // Return `Self::empty` for all zero sized rectangles.
        // The iterator would behave correctly without this check, but would loop unnecessarily for
        // rectangles with zero width.
        if rectangle.is_zero_sized() {
            return Self::empty();
        }

        let x = rectangle.columns();
        let y = rectangle.rows();
        let x_start = x.start;

        Self { x, y, x_start }
    }

    /// Create a points iterator that returns no items.
    pub const fn empty() -> Self {
        Self {
            x: 0..0,
            y: 0..0,
            x_start: 0,
        }
    }
}

impl Iterator for RectanglePixels {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while !self.y.is_empty() {
            if let Some(x) = self.x.next() {
                return Some(IVec2::new(x, self.y.start));
            }

            self.y.next();
            self.x.start = self.x_start;
        }

        None
    }
}


/// Iterator over all IVec2s inside the rectangle.
#[derive(Clone,  Debug)]
pub struct RectangleOutlinePixels {
    scanlines: Scanlines,
    current_scanline: Scanline,
}

impl RectangleOutlinePixels {
    pub(crate) fn new(rectangle: &RectangleIVec2) -> Self {
        Self {
            scanlines: Scanlines::new(rectangle),
            current_scanline: Scanline::new_empty(0),
        }
    }

}

impl Iterator for RectangleOutlinePixels {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current_scanline.next().or_else(|| {
            self.current_scanline = self.scanlines.next()?;
            self.current_scanline.next()
        })
    }
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Scanlines {
    first_row: i32,
    rows: Range<i32>,
    columns: Range<i32>,
}

impl Scanlines {
    pub fn new(rect: &RectangleIVec2) -> Self {
        let rows = rect.rows();
        Self {
            first_row: rows.start,
            rows,
            columns: rect.columns(),
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.rows.next()?;

        if y == self.first_row || y == self.rows.end - 1 { 
            return Some(Scanline::new(y, self.columns.clone(), false));
        } else {
            return Some(Scanline::new(y, self.columns.clone(), true));
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rectangle_pixels() {
        let rect = RectangleIVec2::new(IVec2::new(0, 0), IVec2::new(2, 2));
        let pixels: Vec<IVec2> = RectanglePixels::new(&rect).collect();
        assert_eq!(pixels, vec![
            IVec2::new(0, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ]);
    }

    #[test]
    fn rectangle_outline_pixels() {
        let rect = RectangleIVec2::new(IVec2::new(0, 0), IVec2::new(4, 4));
        let pixels: Vec<IVec2> = RectangleOutlinePixels::new(&rect).collect();
        // top row
        assert!(pixels.contains(&IVec2::new(0, 0)));
        assert!(pixels.contains(&IVec2::new(1, 0)));
        assert!(pixels.contains(&IVec2::new(2, 0)));
        assert!(pixels.contains(&IVec2::new(3, 0)));

        // second row
        assert!(pixels.contains(&IVec2::new(0, 1)));
        assert!(pixels.contains(&IVec2::new(3, 1)));

        // third row
        assert!(pixels.contains(&IVec2::new(0, 2)));
        assert!(pixels.contains(&IVec2::new(3, 2)));

        // fourth row
        assert!(pixels.contains(&IVec2::new(0, 3)));
        assert!(pixels.contains(&IVec2::new(1, 3)));
        assert!(pixels.contains(&IVec2::new(2, 3)));
        assert!(pixels.contains(&IVec2::new(3, 3)));

        assert_eq!(pixels.len(), 12);

    }
}
