
use crate::glam::UVec2;
use crate::glam::IVec2;
use crate::{ iters::circle_iter::CirclePoints, RectangleUVec2 };

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CircleUVec2 {
    pub(crate) pos: UVec2,
    pub(crate) radius: u32,
    pub(crate) limits: Option<RectangleUVec2>,
}

impl CircleUVec2 {
    pub const fn new(pos: UVec2, radius: u32) -> Self {
        Self {
            pos,
            radius,
            limits: None,
        }
    }

    /// Returns the bounding box of the circle, disregarding the limits.
    pub fn bounding_box(&self) -> RectangleUVec2 {
        let r = UVec2::splat(self.radius);
        let tl = self.pos - r;
        let br = self.pos + r;
        RectangleUVec2::new(tl, br)
    }

    // /// Returns the bounding box of the circle, respecting limits.
    pub fn limited_bounding_box(&self) -> RectangleUVec2 {
        if let Some(limits) = self.limits {
            let r = UVec2::splat(self.radius);
            let tl = (self.pos - r).max(limits.tl);
            let br = (self.pos + r).min(limits.br);
            RectangleUVec2::new(tl, br)
        } else {
            self.bounding_box()
        }
    }

    /// Return the center point of the circle scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the circle.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    #[allow(dead_code)]
    pub(crate) fn center_2x(&self) -> UVec2 {
        // The radius scaled up by a factor of 2 is equal to the diamete
        let radius_2x = self.radius * 2 - 1;
        (self.pos - UVec2::splat(self.radius)) * 2 + UVec2::splat(radius_2x)
    }

    /// Returns the threshold for this circles diameter.
    #[allow(dead_code)]
    pub(crate) fn threshold(&self) -> u32 {
        diameter_to_threshold(self.radius * 2)
    }

    pub const fn new_with_limits(pos: UVec2, radius: u32, limits: RectangleUVec2) -> Self {
        Self {
            pos,
            radius,
            limits: Some(limits),
        }
    }

    /// Returns an iterator over the pixels of the circle.
    pub fn pixel_iter(&self, outline: bool) -> CirclePoints {
        
        CirclePoints::new(&self.as_circle_ivec2(), outline)
        
    }
    
    pub fn as_circle_ivec2(&self) -> crate::circle::CircleIVec2 {
        crate::circle::CircleIVec2::new(self.pos.as_ivec2(), self.radius as i32)
    }
    

    
    pub fn as_circle_vec2(&self) -> crate::circle::CircleVec2 {
        crate::circle::CircleVec2::new(self.pos.as_vec2(), self.radius as f32)
    }
    

    
}

impl crate::Shape<UVec2> for CircleUVec2 {
    fn position(&self) -> UVec2 {
        self.pos
    }

    fn center(&self) -> UVec2 {
        self.pos
    }

    fn contains(&self, coord: UVec2) -> bool {
        let delta = self.center_2x() - coord * 2;
        
        let distance = delta.length_squared();

        distance < self.threshold()
    }

    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2> {
        self.pixel_iter(outline)
    }
}

pub(crate) fn diameter_to_threshold(diameter: u32) -> u32 {
    if diameter <= 4 { 
        diameter.pow(2) - diameter / 2
        } else { 
        diameter.pow(2)
        }
}

