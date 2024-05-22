use glam::IVec2;
use crate::iters::line_iter::{LineIter, LineDrawAlgo};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct LineSegment {
    pub start: IVec2,
    pub end: IVec2,
}

impl LineSegment {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        Self { 
            start, 
            end 
        }
    }

    /// Create a new line segment from a start point, a direction and a length
    pub fn new_from_length(start: IVec2, dir: IVec2, length: i32) -> Self {
        Self { start, end: start + dir * length }
    }

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = IVec2> {
        LineIter::new_from_segment(*self, draw_mode)
        
    }

    pub fn length (&self) -> i32 {
        self.start.as_vec2().distance_squared(self.end.as_vec2()).sqrt() as i32
        
    }
}

