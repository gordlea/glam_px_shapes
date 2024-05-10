
use std::ops::Range;

use crate::glam::UVec2;
use crate::glam::IVec2;
use crate::iters::rect_iter::RectanglePixels;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RectangleUVec2 {
    pub tl: UVec2,
    pub br: UVec2,
}

impl RectangleUVec2 {
    pub const fn new_const(tl: UVec2, br: UVec2) -> Self {
        Self { tl, br }
    }

    pub fn new_on_origin(size: UVec2) -> Self {
        Self { tl: UVec2::ZERO, br: size }
    }

    pub fn new(tl: UVec2, br: UVec2) -> Self {

        let ttl = UVec2::new(tl.x.min(br.x), tl.y.min(br.y));
        let tbr = UVec2::new(tl.x.max(br.x), tl.y.max(br.y));

        Self { tl: ttl, br: tbr }
    }

    pub fn new_with_limits(tl: UVec2, br: UVec2, limits: RectangleUVec2) -> Self {
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

    pub fn from_points(points: Vec<UVec2>) -> Self {
        let mut tl = UVec2::new(u32::MAX, u32::MAX);
        let mut br = UVec2::new(u32::MIN, u32::MIN);

        for point in points {
            tl = tl.min(point);
            br = br.max(point);
        }

        Self { tl, br }
    }
    
    // if both points are zero, the rectangle is zeroed
    pub fn is_zeroed(&self) -> bool {
        self.tl == UVec2::ZERO && self.br == UVec2::ZERO
    }

    pub fn add_point(&mut self, point: UVec2) {
        self.tl = self.tl.min(point);
        self.br = self.br.max(point);
    }


    pub fn tl(&self) -> UVec2 {
        self.tl
    }

    pub fn br(&self) -> UVec2 {
        self.br
    }

    pub fn size(&self) -> UVec2 {
        self.br - self.tl
    }

    pub fn position(&self) -> UVec2 {
        self.tl
    }

    pub fn contains(&self, coord: UVec2) -> bool {
        self.tl.cmple(coord).all() && self.br.cmpge(coord).all()
    }    

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows(&self) -> Range<u32> {
        self.tl.y..self.br.y  
    }

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows_limited(&self, limit: &Option<RectangleUVec2>) -> Range<u32> {
        if let Some(limit) = limit {
            self.tl.y.max(limit.tl.y)..self.br.y.min(limit.br.y)
        } else {
            self.rows()
        }
    }

    /// Returns the range of X coordinates in this rectangle.
    pub fn columns(&self) -> Range<u32> {
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
        
        let irect = crate::RectangleIVec2::new(self.tl.as_ivec2(), self.br.as_ivec2());
        RectanglePixels::new(&irect, outline)
        
    }
}


impl crate::Shape<UVec2> for RectangleUVec2 {
    fn position(&self) -> UVec2 {
        self.tl()
    }

    fn center(&self) -> UVec2 {
        (self.tl + self.br) / 2
    }

    fn contains(&self, coord: UVec2) -> bool {
        self.contains(coord)
    }

    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2> {
        self.pixel_iter(outline)
    }
}
