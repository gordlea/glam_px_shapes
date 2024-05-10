
use std::ops::Range;

use crate::glam::Vec2;
use crate::iters::rect_iter::{RectangleOutlinePixels, RectanglePixels};


pub struct RectangleVec2 {
    pub(crate) tl: Vec2,
    pub(crate) br: Vec2,
}


impl RectangleVec2 {
    pub const fn new_const(tl: Vec2, br: Vec2) -> Self {
        Self { tl, br }
    }

    pub fn new_on_origin(size: Vec2) -> Self {
        Self { tl: Vec2::ZERO, br: size }
    }

    pub fn new(tl: Vec2, br: Vec2) -> Self {

        let ttl = Vec2::new(tl.x.min(br.x), tl.y.min(br.y));
        let tbr = Vec2::new(tl.x.max(br.x), tl.y.max(br.y));

        Self { tl: ttl, br: tbr }
    }

    pub fn new_with_limits(tl: Vec2, br: Vec2, limits: RectangleVec2) -> Self {
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


    pub fn tl(&self) -> Vec2 {
        self.tl
    }

    pub fn br(&self) -> Vec2 {
        self.br
    }

    pub fn size(&self) -> Vec2 {
        self.br - self.tl
    }

    pub fn position(&self) -> Vec2 {
        self.tl
    }

    pub fn contains(&self, coord: Vec2) -> bool {
        self.tl.cmple(coord).all() && self.br.cmpge(coord).all()
    }    

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows(&self) -> Range<f32> {
        self.tl.y..self.br.y  
    }

    /// Returns the range of Y coordinates in this rectangle.
    pub fn rows_limited(&self, limit: &Option<RectangleVec2>) -> Range<f32> {
        if let Some(limit) = limit {
            self.tl.y.max(limit.tl.y)..self.br.y.min(limit.br.y)
        } else {
            self.rows()
        }
    }

    /// Returns the range of X coordinates in this rectangle.
    pub fn columns(&self) -> Range<f32> {
        self.tl.x..self.br.x
    }

    /// Returns `true` is the rectangle is zero sized.
    ///
    /// A rectangle is zero sized if the width or height are zero.
    pub fn is_zero_sized(&self) -> bool {
        let size = self.size();
        size.y == 0.0 || size.x == 0.0
    }

    pub fn pixel_iter(&self) -> RectanglePixels {
        
        let irect = crate::rectangle::rectangle_ivec2::RectangleIVec2::new(self.tl.as_ivec2(), self.br.as_ivec2());
        RectanglePixels::new(&irect)
        
    }

    pub fn pixel_outline_iter(&self) -> RectangleOutlinePixels {
        
        let irect = crate::rectangle::rectangle_ivec2::RectangleIVec2::new(self.tl.as_ivec2(), self.br.as_ivec2());
        RectangleOutlinePixels::new(&irect)
        
    }
}


