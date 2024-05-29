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
        let iter = ls.pixel_iter(LineDrawAlgo::Bresenham);

        let results: Vec<IVec2> = iter.collect();
        assert_eq!(results, vec![
            IVec2::new(0, 0),
            IVec2::new(0, 1),
            IVec2::new(-1, 2),
            IVec2::new(-1, 3),
        ]);
    }   
}