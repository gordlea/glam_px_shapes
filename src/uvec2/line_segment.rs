use glam::UVec2;
use glam::IVec2;
use crate::iters::line_iter::{LineIter, LineDrawAlgo};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct LineSegment {
    pub start: UVec2,
    pub end: UVec2,
}

impl LineSegment {
    pub fn new(start: UVec2, end: UVec2) -> Self {
        Self { 
            start, 
            end 
        }
    }

    /// Create a new line segment from a start point, a direction and a length
    pub fn new_from_length(start: UVec2, dir: UVec2, length: u32) -> Self {
        Self { start, end: start + dir * length }
    }

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = IVec2> {
        let iseg = crate::ivec2::LineSegment::new(self.start.as_ivec2(), self.end.as_ivec2());
        LineIter::new_from_segment(iseg, draw_mode)
        
    }

    pub fn length (&self) -> u32 {
        self.start.as_vec2().distance_squared(self.end.as_vec2()).sqrt() as u32
        
    }
}

