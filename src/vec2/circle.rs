use approx::AbsDiffEq;

use glam::Vec2;
use glam::IVec2;
use crate::iters::circle_iter::CirclePoints;
use super::rectangle::Rectangle;

#[derive(Debug, Copy, Clone, Default)]
pub struct Circle {
    pub(crate) pos: Vec2,
    pub(crate) radius: f32,
    pub(crate) limits: Option<Rectangle>,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.pos.abs_diff_eq(other.pos, f32::EPSILON) 
            && self.radius.abs_diff_eq(&other.radius, f32::EPSILON)
            && self.limits == other.limits
    }
}

impl Eq for Circle {}

impl Circle {
    pub const fn new(pos: Vec2, radius: f32) -> Self {
        Self {
            pos,
            radius,
            limits: None,
        }
    }

    /// Returns the bounding box of the circle, disregarding the limits.
    pub fn bounding_box(&self) -> Rectangle {
        let r = Vec2::splat(self.radius);
        let tl = self.pos - r;
        let br = self.pos + r;
        Rectangle::new(tl, br)
    }

    // /// Returns the bounding box of the circle, respecting limits.
    pub fn limited_bounding_box(&self) -> Rectangle {
        if let Some(limits) = self.limits {
            let r = Vec2::splat(self.radius);
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
    pub(crate) fn center_2x(&self) -> Vec2 {
        // The radius scaled up by a factor of 2 is equal to the diamete
        let radius_2x = self.radius * 2.0 - 1.0;
        (self.pos - Vec2::splat(self.radius)) * 2.0 + Vec2::splat(radius_2x)
    }

    /// Returns the threshold for this circles diameter.
    #[allow(dead_code)]
    pub(crate) fn threshold(&self) -> f32 {
        diameter_to_threshold(self.radius * 2.0)
    }

    pub const fn new_with_limits(pos: Vec2, radius: f32, limits: Rectangle) -> Self {
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
    
    pub fn as_circle_ivec2(&self) -> crate::ivec2::Circle {
        crate::ivec2::Circle::new(self.pos.as_ivec2(), self.radius as i32)
    }
    

    

    
    pub fn as_circle_uvec2(&self) -> crate::uvec2::Circle {
        crate::uvec2::Circle::new(self.pos.as_uvec2(), self.radius as u32)
    }
    
}

impl crate::Shape<Vec2> for Circle {
    fn position(&self) -> Vec2 {
        self.pos
    }

    fn center(&self) -> Vec2 {
        self.pos
    }

    fn contains(&self, coord: Vec2) -> bool {
        
        let delta = self.center_2x() - coord * 2.0;
        
        let distance = delta.length_squared();

        distance < self.threshold()
    }

    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2> {
        self.pixel_iter(outline)
    }
}

pub(crate) fn diameter_to_threshold(diameter: f32) -> f32 {
    if diameter <= 4.0 { 
        diameter.powf(2.0) - diameter / 2.0 - 1.0
        } else { 
        diameter.powf(2.0)
        }
}

