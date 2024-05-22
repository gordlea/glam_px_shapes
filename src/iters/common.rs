use std::ops::Range;

use glam::IVec2;

/// Scanline.
#[derive(Debug, Clone)]
pub struct Scanline {
    pub y: i32,
    pub x: Range<i32>,
    // pub x_iter: std::slice::Iter<'a, i32>,
    pub outline: bool,
    pub outline_started: bool,
    pub outline_finished: bool,
}

impl Scanline {
    /// Creates a new scanline.
    pub fn new(y: i32, x: Range<i32>, outline: bool) -> Self {
        Self { y, x: x.into_iter(), outline, outline_started: false, outline_finished: false }
    }

    /// Creates a new empty scanline.
    pub fn new_empty(y: i32) -> Self {
        Self::new(y, 0..0, false)
    }

    /// Returns `true` if the x range of the scanline is empty.
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }
}

impl Iterator for Scanline {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.outline {
            if !self.outline_started {
                self.outline_started = true;
                Some(IVec2::new(self.x.start, self.y))
            } else if !self.outline_finished {
                self.outline_finished = true;
                Some(IVec2::new(self.x.end - 1, self.y))
            } else {
                None
            }
        } else {
            self.x.next().map(|x| IVec2::new(x, self.y))
        }
    }
}


// /// Scanline.
// #[derive(Debug, Clone)]
// pub struct Scanoutline<'a> {
//     pub y: i32,
//     pub x_iter: std::slice::Iter<'a, i32>,
// }

// const EMPTY_SCANOUTLINE_ARRAY: [i32; 2] = [0_i32, 0_i32];

// impl<'a> Scanoutline<'a> {
//     /// Creates a new Scanoutline.
//     pub fn new(y: i32, x: &'a [i32; 2]) -> Self {
//         Self { y, x_iter: x.iter() }
//     }

//     /// Creates a new empty Scanoutline.
//     pub fn new_empty(y: i32) -> Self {
//         Self::new(y, &EMPTY_SCANOUTLINE_ARRAY)
//     }

//     /// Returns `true` if the x range of the Scanoutline is empty.
//     pub fn is_empty(&self) -> bool {
//         self.x_iter.is_empty()
//     }
// }

// impl Iterator for Scanoutline<'_> {
//     type Item = IVec2;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.x_iter.next().map(|x| IVec2::new(self.y, *x))
//     }
// }
