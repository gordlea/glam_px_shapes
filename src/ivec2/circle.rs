
use glam::IVec2;
use crate::iters::circle_iter::CirclePoints;
use super::rectangle::Rectangle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Circle {
    pub(crate) pos: IVec2,
    pub(crate) radius: i32,
    pub(crate) limits: Option<Rectangle>,
}

impl Circle {
    pub const fn new(pos: IVec2, radius: i32) -> Self {
        Self {
            pos,
            radius,
            limits: None,
        }
    }

    /// Returns the bounding box of the circle, disregarding the limits.
    pub fn bounding_box(&self) -> Rectangle {
        let r = IVec2::splat(self.radius);
        let tl = self.pos - r;
        let br = self.pos + r;
        Rectangle::new(tl, br)
    }

    // /// Returns the bounding box of the circle, respecting limits.
    pub fn limited_bounding_box(&self) -> Rectangle {
        if let Some(limits) = self.limits {
            let r = IVec2::splat(self.radius);
            let tl = (self.pos - r).max(limits.tl);
            let br = (self.pos + r).min(limits.br);
            Rectangle::new(tl, br)
        } else {
            self.bounding_box()
        }
    }

    /// Return the center point of the circle scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the circle.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    #[allow(dead_code)]
    pub(crate) fn center_2x(&self) -> IVec2 {
        // The radius scaled up by a factor of 2 is equal to the diamete
        let radius_2x = self.radius * 2 - 1;
        (self.pos - IVec2::splat(self.radius)) * 2 + IVec2::splat(radius_2x)
    }

    /// Returns the threshold for this circles diameter.
    #[allow(dead_code)]
    pub(crate) fn threshold(&self) -> i32 {
        diameter_to_threshold(self.radius * 2)
    }

    pub const fn new_with_limits(pos: IVec2, radius: i32, limits: Rectangle) -> Self {
        Self {
            pos,
            radius,
            limits: Some(limits),
        }
    }

    /// Returns an iterator over the pixels of the circle.
    pub fn pixel_iter(&self, outline: bool) -> CirclePoints {
        
        CirclePoints::new(self, outline)
        
    }
    

    
    pub fn as_circle_vec2(&self) -> crate::vec2::Circle {
        crate::vec2::Circle::new(self.pos.as_vec2(), self.radius as f32)
    }
    

    
    pub fn as_circle_uvec2(&self) -> crate::uvec2::Circle {
        crate::uvec2::Circle::new(self.pos.as_uvec2(), self.radius as u32)
    }
    
}

impl crate::Shape<IVec2> for Circle {
    fn position(&self) -> IVec2 {
        self.pos
    }

    fn center(&self) -> IVec2 {
        self.pos
    }

    fn contains(&self, coord: IVec2) -> bool {
        let delta = self.center_2x() - coord * 2;
        
        let distance = delta.length_squared();

        distance < self.threshold()
    }

    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2> {
        self.pixel_iter(outline)
    }
}

pub(crate) fn diameter_to_threshold(diameter: i32) -> i32 {
    if diameter <= 4 { 
        diameter.pow(2) - diameter / 2
        } else { 
        diameter.pow(2)
        }
}

