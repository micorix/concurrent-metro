use crate::common::structs::{Coords, Direction, PathElement};

pub struct PathIterator<'a> {
    path: &'a [Coords],
    index: usize,
    direction: Direction,
}

impl<'a> PathIterator<'a> {
    pub fn new(path: &'a [Coords]) -> Self {
        PathIterator {
            path,
            index: 0,
            direction: Direction::Forward,
        }
    }

    pub fn peek(&self, steps: i32) -> Option<PathElement> {
        if self.path.is_empty() {
            return None;
        }

        let mut idx = self.index as i32;
        let mut dir = if steps >= 0 {
            self.direction
        } else {
            match self.direction {
                Direction::Forward => Direction::Backward,
                Direction::Backward => Direction::Forward,
            }
        };

        for _ in 0..steps.abs() {
            idx += match dir {
                Direction::Forward => 1,
                Direction::Backward => -1,
            };

            if idx >= self.path.len() as i32 {
                idx = (self.path.len() - 1) as i32;
                dir = Direction::Backward;
            } else if idx < 0 {
                idx = 0;
                dir = Direction::Forward;
            }
        }

        Some(PathElement {
            index: idx as usize,
            coords: self.path[idx as usize],
        })
    }

    pub fn has_next(&self) -> bool {
        !self.path.is_empty()
    }
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = PathElement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.is_empty() {
            return None;
        }

        let element = PathElement {
            index: self.index,
            coords: self.path[self.index],
        };

        // Move index in ping-pong manner
        match self.direction {
            Direction::Forward => {
                if self.index + 1 < self.path.len() {
                    self.index += 1;
                } else {
                    self.direction = Direction::Backward;
                    if self.index > 0 {
                        self.index -= 1;
                    }
                }
            }
            Direction::Backward => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.direction = Direction::Forward;
                    if self.index + 1 < self.path.len() {
                        self.index += 1;
                    }
                }
            }
        }

        Some(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_path() -> Vec<Coords> {
        vec![
            Coords::new(0, 0),
            Coords::new(1, 1),
            Coords::new(2, 2),
            Coords::new(3, 3),
        ]
    }
    
    fn collect_path_indices(mut iter: PathIterator, end: usize) -> Vec<usize> {
        (0..end).map(|_| iter.next().unwrap().index).collect()
    }

    #[test]
    fn test_forward_iteration() {
        let path = sample_path();
        let iter = PathIterator::new(&path);
        
        assert_eq!(collect_path_indices(iter, 4), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test__looping() {
        let path = sample_path();
        let iter = PathIterator::new(&path);
        
        assert_eq!(collect_path_indices(iter, 10), vec![0, 1, 2, 3, 2, 1, 0, 1, 2, 3]);
    }

    #[test]
    fn test_peek_forward() {
        let path = sample_path();
        let iter = PathIterator::new(&path);

        let peeked = iter.peek(2).unwrap();
        assert_eq!(peeked.index, 2);
        assert_eq!(peeked.coords, Coords::new(2, 2));
    }

    #[test]
    fn test_peek_backward() {
        let path = sample_path();
        let mut iter = PathIterator::new(&path);
        
        for _ in 0..3 {
            iter.next();
        }

        let peeked = iter.peek(-2).unwrap();
        assert_eq!(peeked.index, 1);
        assert_eq!(peeked.coords, Coords::new(1, 1));
    }

    #[test]
    fn test_looping_back_to_start() {
        let path = sample_path();
        let iter = PathIterator::new(&path);

        assert_eq!(collect_path_indices(iter, 7), vec![0, 1, 2, 3, 2, 1, 0]);
    }

    #[test]
    fn test_empty_path() {
        let path: Vec<Coords> = vec![];
        let mut iter = PathIterator::new(&path);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.peek(1), None);
        assert_eq!(iter.has_next(), false);
    }
}
