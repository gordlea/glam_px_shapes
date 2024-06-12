use glam::Vec2;
use strum::EnumDiscriminants;

use crate::vec2::LineSegment;


#[derive(Debug, Copy, Clone)]
pub struct LineIter {
    impl_iter: LineIterImpl,
}



impl LineIter {
    pub fn new_from_segment(ls: LineSegment, iter_type: LineDrawAlgo) -> LineIter {
        match iter_type {
            LineDrawAlgo::DDA => {
                let dx = ls.end.x - ls.start.x;
                let dy = ls.end.y - ls.start.y;
            
                let steps = if dx.abs() > dy.abs() {
                    dx.abs()
                } else {
                    dy.abs()
                };


                let step = Vec2::new(dx / steps, dy / steps);
                let steps = steps.ceil() as i32;
                
                LineIter {
                    impl_iter: LineIterImpl::DDA(DDAParams {
                        // start: ls.start,
                        end: ls.end,
                        current: ls.start,
                        step,
                        steps,
                        current_step: 0,
                    }),
                }
            },
        }
    }


}
impl Iterator for LineIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        self.impl_iter.next()
    }
}


#[derive(Debug, Copy, Clone, Default)]
pub struct DDAParams {
    // start: Vec2,
    end: Vec2,
    current: Vec2,
    step: Vec2,
    steps: i32,
    current_step: i32,
}



#[derive(EnumDiscriminants, Debug, Copy, Clone)]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(LineDrawAlgo))]
pub(crate) enum LineIterImpl {
    DDA(DDAParams),
}

impl Iterator for LineIterImpl {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LineIterImpl::DDA(params) => {
                if params.current_step > params.steps {
                    return None;
                }

                let current = params.current;

                if params.current_step == params.steps {
                    params.current += params.step;
                    params.current_step += 1;
                    return Some(params.end);
                }

                params.current += params.step;
                params.current_step += 1;

                Some(current)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::vec;

    // use crate::vec2;

    use super::*;

    #[test]
    fn test_dda_line_iter_impl_down() {
        let ls = LineSegment::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, 5.0));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

        let expected = vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(1.0, 3.0),
            Vec2::new(1.0, 4.0),
            Vec2::new(1.0, 5.0),
        ];

        assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    }

    #[test]
    fn test_dda_short() {
        let ls = LineSegment::new(Vec2::new(0.725, 0.843), Vec2::new(1.025, 1.143));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

        let expected = vec![
            Vec2::new(0.725, 0.843),
            Vec2::new(1.025, 1.143)
        ];

        assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    }

    #[test]
    fn test_dda_line_iter_impl() {
        let ls = LineSegment::new(Vec2::new(1.0, 1.0), Vec2::new(11.0, 5.0));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

        let expected = vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 1.4),
            Vec2::new(3.0, 1.8),
            Vec2::new(4.0, 2.2),
            Vec2::new(5.0, 2.6000001),
            Vec2::new(6.0, 3.0000002),
            Vec2::new(7.0, 3.4000003),
            Vec2::new(8.0, 3.8000004),
            Vec2::new(9.0, 4.2000003),
            Vec2::new(10.0, 4.6000004),
            Vec2::new(11.0, 5.0000005)
        ];

        assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    }

    #[test]
    fn test_dda_line_iter_impl_going_left() {
        let ls = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(-5.0, 5.0));
        let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

        let expected = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(-2.0, 2.0),
            Vec2::new(-3.0, 3.0),
            Vec2::new(-4.0, 4.0),
            Vec2::new(-5.0, 5.0),
        ];

        assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    }

    // #[test]
    // fn test_dda_line_iter_impl_going_right_slow() {
    //     let ls = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 5.0));
    //     let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

    //     let expected = vec![
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(0.0, 1.0),
    //         Vec2::new(1.0, 2.0),
    //         Vec2::new(1.0, 3.0),
    //         Vec2::new(1.0, 4.0),
    //     ];

    //     assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    // }    

    // #[test]
    // fn test_dda_line_iter_impl_going_left_slow() {
    //     let ls = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(-2.0, 5.0));
    //     let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

    //     let expected = vec![
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(-1.0, 1.0),
    //         Vec2::new(-2.0, 2.0),
    //         Vec2::new(-3.0, 3.0),
    //         Vec2::new(-4.0, 4.0),
    //         Vec2::new(-5.0, 5.0),
    //     ];

    //     assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    // }    

    // #[test]
    // fn test_dda_line_iter_impl_diag() {
    //     let ls = LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 3.0));
    //     let iter = LineIter::new_from_segment(ls, LineDrawAlgo::DDA);

    //     let expected = vec![
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(1.0, 1.0),
    //         Vec2::new(2.0, 2.0),
    //         Vec2::new(3.0, 3.0),
    //     ];
    //     assert_eq!(iter.collect::<Vec<Vec2>>(), expected);

    // }    


    // #[test]
    // fn test_pixel_iterator() {
    //     let ls = vec2::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(3.0, 3.0));
    //     let mut iter = ls.pixel_iter(LineDrawAlgo::DDA);
    //     assert_eq!(iter.next(), Some(Vec2::new(0.0, 0.0)));
    //     assert_eq!(iter.next(), Some(Vec2::new(1.0, 1.0)));
    //     assert_eq!(iter.next(), Some(Vec2::new(2.0, 2.0)));
    //     assert_eq!(iter.next(), Some(Vec2::new(3.0, 3.0)));
    //     assert_eq!(iter.next(), None);
    // }


    // #[test]
    // fn test_pixel_iterator_going_left() {
    //     let ls = vec2::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(-1.0, 10.0));
    //     let iter = ls.pixel_iter(LineDrawAlgo::DDA);

    //     let results: Vec<Vec2> = iter.collect();
    //     assert_eq!(results, vec![
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(0.0, 1.0),
    //         Vec2::new(-1.0, 2.0),
    //         Vec2::new(-1.0, 3.0),
    //     ]);
    // }   
}