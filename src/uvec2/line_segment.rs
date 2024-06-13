use crate::ivec2::{LineIter, LineDrawAlgo};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct LineSegment {
    pub start: glam::UVec2,
    pub end: glam::UVec2,
}

impl LineSegment {
    pub fn new(start: glam::UVec2, end: glam::UVec2) -> Self {
        Self { 
            start, 
            end 
        }
    }

    /// Create a new line segment from a start point, a direction and a length
    pub fn new_from_length(start: glam::UVec2, dir: glam::UVec2, length: u32) -> Self {
        Self { start, end: start + dir * length }
    }

    pub fn pixel_iter(&self, draw_mode: LineDrawAlgo) -> impl Iterator<Item = glam::IVec2> {
    
        let iseg = crate::ivec2::LineSegment::new(self.start.as_ivec2(), self.end.as_ivec2());
        LineIter::new_from_segment(iseg, draw_mode)
        
    }

    

    pub fn length (&self) -> u32 {
        self.start.as_vec2().distance_squared(self.end.as_vec2()).sqrt() as u32
        
    }
    
    pub fn as_vec2(&self) -> crate::vec2::LineSegment {
        crate::vec2::LineSegment {
            start: self.start.as_vec2(),
            end: self.end.as_vec2()
        }
    }
    
    
    pub fn as_ivec2(&self) -> crate::ivec2::LineSegment {
        crate::ivec2::LineSegment {
            start: self.start.as_ivec2(),
            end: self.end.as_ivec2()
        }
    }
    }

