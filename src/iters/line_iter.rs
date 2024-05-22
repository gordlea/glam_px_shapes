use std::default;

use glam::{IVec2, Vec2};
use strum::EnumDiscriminants;

use crate::ivec2::LineSegment;


#[derive(Debug, Copy, Clone)]
pub struct LineIter {
    impl_iter: LineIterImpl,
}



impl LineIter {
    pub fn new_from_segment(ls: LineSegment, iter_type: LineDrawAlgo) -> LineIter {
        match iter_type {
            LineDrawAlgo::WalkGrid => {
                let d = ls.end - ls.start;
                let point = ls.start;
                let i = Vec2::ZERO;
                let sign = d.signum();
                let n = d.abs().as_vec2();

                LineIter {
                    impl_iter: LineIterImpl::WalkGrid(WalkGridParams {
                        point,
                        i,
                        sign,
                        n,
                    })
                }
            }
            // LineDrawAlgo::Bresenham => {
            //     let octant = Octant::new(ls.start, ls.end);
            //     let start = octant.to(ls.start);
            //     let end = octant.to(ls.end);
            //     let delta = end - start;

            //     LineIter {
            //         impl_iter: LineIterImpl::Bresenham(BresenhamParams {
            //             point: start,
            //             end_x: end.x,
            //             delta,
            //             error: delta.y - delta.x,
            //             octant,
            //         })
            //     }
            // },
            LineDrawAlgo::Bresenham => {
                let dx = i32::abs(ls.end.x - ls.start.x);
                let dy = i32::abs(ls.end.y - ls.start.y);
                let error = (if dx > dy { dx } else { -dy }) / 2;
                let sx = {
                    if ls.start.x < ls.end.x {
                        1
                    } else {
                        -1
                    }
                };
                let sy = {
                    if ls.start.y < ls.end.y {
                        1
                    } else {
                        -1
                    }
                };
                
                LineIter {
                    impl_iter: LineIterImpl::Bresenham(BresenhamParams {
                        end: ls.end,
                        current_x: ls.start.x,
                        current_y: ls.start.y,
                        dx,
                        dy,
                        error,
                        sx,
                        sy,
                        done: false,
                    })
                }
            },
        }
    }
}
impl Iterator for LineIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        self.impl_iter.next()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WalkGridParams {
    point: IVec2,
    i: Vec2,
    sign: IVec2,
    n: Vec2,
}

// #[derive(Debug, Copy, Clone)]
// pub struct BresenhamParams {
//     point: IVec2,
//     end_x: i32,
//     delta: IVec2,
//     error: i32,
//     octant: Octant,
// }

#[derive(Debug, Copy, Clone, Default)]
pub struct BresenhamParams {
    end: IVec2,
    current_x: i32,
    current_y: i32,
    dx: i32,
    dy: i32,
    error: i32,
    sx: i32,
    sy: i32,
    done: bool,
}



#[derive(EnumDiscriminants, Debug, Copy, Clone)]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(LineDrawAlgo))]
pub(crate) enum LineIterImpl {
    WalkGrid(WalkGridParams),
    // Bresenham(BresenhamParams),
    Bresenham(BresenhamParams),
}

impl Iterator for LineIterImpl {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LineIterImpl::WalkGrid(params) => {
                if params.i.x <= params.n.x && params.i.y <= params.n.y {
                    let point = params.point;
        
                    if (0.5 + params.i.x) / params.n.x < (0.5 + params.i.y) / params.n.y {
                        params.point.x += params.sign.x;
                        params.i.x += 1.0;
                    } else {
                        params.point.y += params.sign.y;
                        params.i.y += 1.0;
                    }
        
                    Some(point)
                } else {
                    None
                }
            }
            // LineIterImpl::Bresenham(params) => {
            //     if params.point.x <= params.end_x {
            //         let point = params.octant.from(params.point);
        
            //         if params.error >= 0 {
            //             params.point.y += 1;
            //             params.error -= params.delta.x;
            //         }
        
            //         params.point.x += 1;
            //         params.error += params.delta.y;
        
            //         Some(point)
            //     } else {
            //         None
            //     }
            // },
            LineIterImpl::Bresenham(params) => {
                if params.done {
                    return None;
                }
                let current = IVec2::new(params.current_x, params.current_y);

                if params.current_x == params.end.x && params.current_y == params.end.y {
                    params.done = true;
                    return Some(current);
                }
                let error2 = params.error;

                if error2 > -params.dx {
                    params.error -= params.dy;
                    params.current_x += params.sx;
                }
                if error2 < params.dy {
                    params.error += params.dx;
                    params.current_y += params.sy;
                }



                Some(current)
            }
        }
    }
}








// /// A simple octant struct for transforming line points.
// #[derive(Debug, Copy, Clone)]
// pub struct Octant {
//     value: u8,
// }

// impl Octant {
//     #[inline]
//     /// Get the relevant octant from a start and end point.
//     pub fn new(start: IVec2, end: IVec2) -> Self
//     {
//         let mut value = 0;
//         let mut delta = end - start;

//         if delta.y < 0 {
//             delta = -delta;
//             value += 4;
//         }

//         if delta.x < 0 {

//             std::mem::swap(&mut delta.x, &mut delta.y);
//             delta.y = -delta.y;


//             value += 2
//         }

//         if delta.x < delta.y {
//             value += 1
//         }

//         Self { value }
//     }

//     /// Convert a point to its position in the octant.
//     #[inline]
//     pub fn to(&self, point: IVec2) -> IVec2 {
//         match self.value {
//             0 => point,
//             1 => IVec2::new(point.y, point.x),
//             2 => IVec2::new(point.y, -point.x),
//             3 => IVec2::new(-point.x, point.y),
//             4 => IVec2::new(-point.x, -point.y),
//             5 => IVec2::new(-point.y, -point.x),
//             6 => IVec2::new(-point.y, point.x),
//             7 => IVec2::new(point.x, -point.y),
//             _ => unreachable!(),
//         }
//     }

//     /// Convert a point from its position in the octant.
//     #[inline]
//     pub fn from(&self, point: IVec2) -> IVec2 {
//         match self.value {
//             0 => point,
//             1 => IVec2::new(point.y, point.x),
//             2 => IVec2::new(-point.y, point.x),
//             3 => IVec2::new(-point.x, point.y),
//             4 => IVec2::new(-point.x, -point.y),
//             5 => IVec2::new(-point.y, -point.x),
//             6 => IVec2::new(point.y, -point.x),
//             7 => IVec2::new(point.x, -point.y),
//             _ => unreachable!(),
//         }
//     }
// }


#[derive(Debug, Copy, Clone, Default)]
struct Octant2(u8);

impl Octant2 {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    fn from_points(start: IVec2, end: IVec2) -> Octant2 {

        let mut d = end - start;

        let mut octant = 0;

        if d.y < 0 {
            d *= d.signum();
            // dx = -dx;
            // dy = -dy;
            octant += 4;
        }

        if d.x < 0 {
            let tmp = d.x;
            d.x = d.y;
            d.y = -tmp;
            octant += 2
        }

        if d.x < d.y {
            octant += 1
        }

        Octant2(octant)
    }

    #[inline]
    fn to_octant0(&self, p: IVec2) -> IVec2 {
        match self.0 {
            0 => p,
            1 => IVec2::new(p.y, p.x),
            2 => IVec2::new(p.y, -p.x),
            3 => IVec2::new(-p.x, p.y),
            4 => IVec2::new(-p.x, -p.y),
            5 => IVec2::new(-p.y, -p.x),
            6 => IVec2::new(-p.y, p.x),
            7 => IVec2::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn from_octant0(&self, p: IVec2) -> IVec2 {
        match self.0 {
            0 => p,
            1 => IVec2::new(p.y, p.x),
            2 => IVec2::new(-p.y, p.x),
            3 => IVec2::new(-p.x, p.y),
            4 => IVec2::new(-p.x, -p.y),
            5 => IVec2::new(-p.y, -p.x),
            6 => IVec2::new(p.y, -p.x),
            7 => IVec2::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }
}




#[cfg(test)]
mod test {
    use std::vec;

    use crate::vec2;

    use super::*;

    #[test]
    fn test_bresenham_line_iter_impl_down() {
        let ls = LineSegment::new(IVec2::new(1, 1), IVec2::new(1, 5));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(1, 1),
            IVec2::new(1, 2),
            IVec2::new(1, 3),
            IVec2::new(1, 4),
            IVec2::new(1, 5),
        ];

        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }

    #[test]
    fn test_bresenham_line_iter_impl() {
        let ls = LineSegment::new(IVec2::new(1, 1), IVec2::new(11, 5));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(1, 1),
            IVec2::new(2, 1),
            IVec2::new(3, 2),
            IVec2::new(4, 2),
            IVec2::new(5, 3),
            IVec2::new(6, 3),
            IVec2::new(7, 3),
            IVec2::new(8, 4),
            IVec2::new(9, 4),
            IVec2::new(10, 5),
            IVec2::new(11, 5),
        ];

        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }

    #[test]
    fn test_bresenham_line_iter_impl_going_left() {
        let ls = LineSegment::new(IVec2::new(0, 0), IVec2::new(-5, 5));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(0, 0),
            IVec2::new(-1, 1),
            IVec2::new(-2, 2),
            IVec2::new(-3, 3),
            IVec2::new(-4, 4),
            IVec2::new(-5, 5),
        ];

        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }

    #[test]
    fn test_bresenham_line_iter_impl_going_right_slow() {
        let ls = LineSegment::new(IVec2::new(0, 0), IVec2::new(1, 5));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(0, 0),
            IVec2::new(0, 1),
            IVec2::new(1, 2),
            IVec2::new(1, 3),
            IVec2::new(1, 4),
        ];

        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }    

    #[test]
    fn test_bresenham_line_iter_impl_going_left_slow() {
        let ls = LineSegment::new(IVec2::new(0, 0), IVec2::new(-2, 5));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(0, 0),
            IVec2::new(-1, 1),
            IVec2::new(-2, 2),
            IVec2::new(-3, 3),
            IVec2::new(-4, 4),
            IVec2::new(-5, 5),
        ];

        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }    

    #[test]
    fn test_bresenham_line_iter_impl_diag() {
        let ls = LineSegment::new(IVec2::new(0, 0), IVec2::new(3, 3));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::Bresenham);

        let expected = vec![
            IVec2::new(0, 0),
            IVec2::new(1, 1),
            IVec2::new(2, 2),
            IVec2::new(3, 3),
        ];
        assert_eq!(iter.collect::<Vec<IVec2>>(), expected);

    }    


    #[test]
    fn test_pixel_iterator() {
        let ls = vec2::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 3.0));
        let mut iter = ls.pixel_iter(LineDrawAlgo::Bresenham);
        assert_eq!(iter.next(), Some(IVec2::new(0, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(3, 3)));
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn test_pixel_iterator_going_left() {
        let ls = vec2::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(-1.0, 10.0));
        let mut iter = ls.pixel_iter(LineDrawAlgo::Bresenham);

        let results: Vec<IVec2> = iter.collect();
        assert_eq!(results, vec![
            IVec2::new(0, 0),
            IVec2::new(0, 1),
            IVec2::new(-1, 2),
            IVec2::new(-1, 3),
        ]);
    }   
}