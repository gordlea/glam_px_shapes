
use std::ops::Range;

use glam::IVec2;
use crate::iters::rect_iter::RectanglePixels;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Rectangle {
    pub tl: IVec2,
    pub br: IVec2,
}

impl Rectangle {
    pub const fn new_const(tl: IVec2, br: IVec2) -> Self {
        Self { tl, br }
    }

    pub fn new_on_origin(size: IVec2) -> Self {
        Self { tl: IVec2::ZERO, br: size }
    }
    #[cfg(feature = "nightly")]
    pub const fn new(tl: IVec2, br: IVec2) -> Self {
        let ttl = IVec2::new(
            if tl.x < br.x { tl.x } else { br.x },
            if tl.y < br.y { tl.y } else { br.y }
        );

        let tbr = IVec2::new(
            if tl.x > br.x { tl.x } else { br.x },
            if tl.y > br.y { tl.y } else { br.y }
        );

        Self { tl: ttl, br: tbr }
    }

    #[cfg(not(feature = "nightly"))]
    pub fn new(tl: IVec2, br: IVec2) -> Self {

        let ttl = IVec2::new(tl.x.min(br.x), tl.y.min(br.y));
        let tbr = IVec2::new(tl.x.max(br.x), tl.y.max(br.y));

        Self { tl: ttl, br: tbr }
    }

    pub fn new_with_limits(tl: IVec2, br: IVec2, limits: Rectangle) -> Self {
        let mut new_tl = tl;
        if tl.x < limits.tl.x {
            new_tl.x = limits.tl.x;
        }
        if tl.y < limits.tl.y {
            new_tl.y = limits.tl.y;
        }
        let mut new_br = br;

        if br.x > limits.br.x {
            new_br.x = limits.br.x;
        }
        if br.y > limits.br.y {
            new_br.y = limits.br.y;
        }

        Self { tl: new_tl, br: new_br }
    }

    pub fn from_points(points: Vec<IVec2>) -> Self {
        let mut tl = IVec2::new(i32::MAX, i32::MAX);
        let mut br = IVec2::new(i32::MIN, i32::MIN);

        for point in points {
            tl = tl.min(point);
            br = br.max(point);
        }

        Self { tl, br }
    }
    
    // if both points are zero, the rectangle is zeroed
    pub fn is_zeroed(&self) -> bool {
        self.tl == IVec2::ZERO && self.br == IVec2::ZERO
    }

    pub fn add_point(&mut self, point: IVec2) {
        self.tl = self.tl.min(point);
        self.br = self.br.max(point);
    }


    pub fn tl(&self) -> IVec2 {
        self.tl
    }

    pub fn tr(&self) -> IVec2 {
        IVec2::new(self.br.x, self.tl.y)
    }    

    pub fn br(&self) -> IVec2 {
        self.br
    }

    pub fn bl(&self) -> IVec2 {
        IVec2::new(self.tl.x, self.br.y)
    }

    pub fn size(&self) -> IVec2 {
        self.br - self.tl
    }

    pub fn position(&self) -> IVec2 {
        self.tl
    }

    pub fn contains(&self, coord: IVec2) -> bool {
        self.tl.cmple(coord).all() && self.br.cmpge(coord).all()
    }    

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.tl.x < other.br.x && self.br.x > other.tl.x && self.tl.y < other.br.y && self.br.y > other.tl.y
    }

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows(&self) -> Range<i32> {
        self.tl.y..self.br.y  
    }

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows_limited(&self, limit: &Option<Rectangle>) -> Range<i32> {
        if let Some(limit) = limit {
            self.tl.y.max(limit.tl.y)..self.br.y.min(limit.br.y)
        } else {
            self.rows()
        }
    }

    /// Returns the range of X coordinates in this rectangle.
    pub fn columns(&self) -> Range<i32> {
        self.tl.x..self.br.x
    }

    /// Returns `true` is the rectangle is zero sized.
    ///
    /// A rectangle is zero sized if the width or height are zero.
    pub fn is_zero_sized(&self) -> bool {
        let size = self.size();
        size.y == 0 || size.x == 0
    }

    pub fn pixel_iter(&self, outline: bool) -> RectanglePixels {
        
        RectanglePixels::new(self, outline)
        
    }
}


impl crate::Shape<IVec2> for Rectangle {
    fn position(&self) -> IVec2 {
        self.tl()
    }

    fn center(&self) -> IVec2 {
        (self.tl + self.br) / 2
    }

    fn contains(&self, coord: IVec2) -> bool {
        self.contains(coord)
    }

    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2> {
        self.pixel_iter(outline)
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut rect = Rectangle::new(IVec2::new(1, 2), IVec2::new(3, 4));
        assert_eq!(rect.tl, IVec2::new(1, 2));
        assert_eq!(rect.br, IVec2::new(3, 4));

        rect = Rectangle::new(IVec2::new(3, 4), IVec2::new(1, 2));
        assert_eq!(rect.tl, IVec2::new(1, 2));
        assert_eq!(rect.br, IVec2::new(3, 4));

        rect = Rectangle::new(IVec2::new(3, 2), IVec2::new(1, 4));
        assert_eq!(rect.tl, IVec2::new(1, 2));
        assert_eq!(rect.br, IVec2::new(3, 4));
    }
}