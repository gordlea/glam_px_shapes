use glam::Vec2;
use glam::IVec2;
use crate::iters::line_iter::{LineIter, LineDrawAlgo};


#[derive(Debug, Copy, Clone, Default)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

impl LineSegment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { 
            start, 
            end 
        }
    }

    /// Create a new line segment from a start point, a direction and a length
    pub fn new_from_length(start: Vec2, dir: Vec2, length: f32) -> Self {
        Self { start, end: start + dir * length }
    }

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = IVec2> {
        let iseg = crate::ivec2::LineSegment::new(self.start.as_ivec2(), self.end.as_ivec2());
        LineIter::new_from_segment(iseg, draw_mode)
        
    }

    pub fn length (&self) -> f32 {
        self.start.distance_squared(self.end).sqrt()
        
    }
}

