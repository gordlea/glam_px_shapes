use glam::Vec2;
use glam::IVec2;
use crate::vec2::{LineIter, LineDrawAlgo};
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

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = Vec2> {
    LineIter::new_from_segment(*self, draw_mode)
        
    }

    

    pub fn length (&self) -> f32 {
        self.start.distance_squared(self.end).sqrt()
        
    }
    
    
    pub fn as_ivec2(&self) -> crate::ivec2::LineSegment {
        crate::ivec2::LineSegment {
            start: self.start.as_ivec2(),
            end: self.end.as_ivec2()
        }
    }
    
    pub fn as_uvec2(&self) -> crate::uvec2::LineSegment {
        crate::uvec2::LineSegment {
            start: self.start.as_uvec2(),
            end: self.end.as_uvec2()
        }
    }
    }

