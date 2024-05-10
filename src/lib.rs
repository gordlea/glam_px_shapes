#![feature(const_trait_impl)]
#![feature(exact_size_is_empty)]
#![feature(const_fn_floating_point_arithmetic)]
mod glam;
mod rectangle;
mod circle;
pub mod iters;

pub use circle::*;
use crate::glam::IVec2;
pub use rectangle::*;



pub trait Shape<T> {
    fn position(&self) -> T;
    fn center(&self) -> T;
    fn contains(&self, coord: T) -> bool;
    fn pixel_iter(&self, outline: bool) -> impl Iterator<Item = IVec2>;
}

