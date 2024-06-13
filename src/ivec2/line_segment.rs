use crate::ivec2::{LineIter, LineDrawAlgo};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct LineSegment {
    pub start: glam::IVec2,
    pub end: glam::IVec2,
}

impl LineSegment {
    pub fn new(start: glam::IVec2, end: glam::IVec2) -> Self {
        Self { 
            start, 
            end 
        }
    }

    /// Create a new line segment from a start point, a direction and a length
    pub fn new_from_length(start: glam::IVec2, dir: glam::IVec2, length: i32) -> Self {
        Self { start, end: start + dir * length }
    }

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = glam::IVec2> {
    LineIter::new_from_segment(*self, draw_mode)
        
    }

    

    pub fn length (&self) -> i32 {
        self.start.as_vec2().distance_squared(self.end.as_vec2()).sqrt() as i32
        
    }
    
    pub fn as_vec2(&self) -> crate::vec2::LineSegment {
        crate::vec2::LineSegment {
            start: self.start.as_vec2(),
            end: self.end.as_vec2()
        }
    }
    
    
    pub fn as_uvec2(&self) -> crate::uvec2::LineSegment {
        crate::uvec2::LineSegment {
            start: self.start.as_uvec2(),
            end: self.end.as_uvec2()
        }
    }
    }

