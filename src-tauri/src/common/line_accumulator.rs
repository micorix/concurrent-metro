use crate::common::structs::Coords;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordsInLineResult {
    Inconclusive,
    Straight,
    NotStraight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineAxis {
    X,
    Y,
}

pub struct LineAccumulator {
    coords: Vec<Coords>,
    line_axis: Option<LineAxis>,
    axis_value: Option<u32>,
}

impl LineAccumulator {
    pub fn new() -> Self {
        Self {
            coords: Vec::new(),
            line_axis: None,
            axis_value: None,
        }
    }

    pub fn from_peek_fn<F>(mut peek_fn: F) -> Self
    where F: FnMut(i32) -> Option<Coords> {
        let mut acc = LineAccumulator::new();
        let mut step = 0;

        while let Some(c) = peek_fn(step) {
            match acc.add_coords(c) {
                CoordsInLineResult::NotStraight => break,
                _ => step += 1,
            }
        }

        acc
    }

    pub fn add_coords(&mut self, coord: Coords) -> CoordsInLineResult {
        if self.coords.is_empty() {
            self.coords.push(coord);
            CoordsInLineResult::Inconclusive
        } else if self.can_be_in_straight_line(&coord) {
            self.coords.push(coord);
            CoordsInLineResult::Straight
        } else {
            CoordsInLineResult::NotStraight
        }
    }

    pub fn collect(&self) -> Vec<Coords> {
        if self.coords.len() <= 1 {
            Vec::new()
        } else {
            self.coords.clone()
        }
    }

    fn can_be_in_straight_line(&mut self, coord: &Coords) -> bool {
        let first = &self.coords[0];

        match self.line_axis {
            None => {
                if first.x == coord.x {
                    self.line_axis = Some(LineAxis::X);
                    self.axis_value = Some(first.x);
                    true
                } else if first.y == coord.y {
                    self.line_axis = Some(LineAxis::Y);
                    self.axis_value = Some(first.y);
                    true
                } else {
                    false
                }
            }
            Some(LineAxis::X) => coord.x == self.axis_value.unwrap(),
            Some(LineAxis::Y) => coord.y == self.axis_value.unwrap(),
        }
    }
}

#[cfg(test)]
mod peek_fn_tests {
    use super::*;

    #[test]
    fn test_from_peek_fn_straight_x() {
        let path = vec![Coords::new(1, 0), Coords::new(1, 1), Coords::new(1, 2), Coords::new(2, 2)];
        let mut step = 0;

        let acc = LineAccumulator::from_peek_fn(|_| {
            let res = path.get(step).copied();
            step += 1;
            res
        });

        let collected = acc.collect();
        // Should collect only the straight x-axis coords
        assert_eq!(collected, vec![Coords::new(1, 0), Coords::new(1, 1), Coords::new(1, 2)]);
    }

    #[test]
    fn test_from_peek_fn_straight_y() {
        let path = vec![Coords::new(0, 5), Coords::new(1, 5), Coords::new(2, 5), Coords::new(2, 6)];
        let mut step = 0;

        let acc = LineAccumulator::from_peek_fn(|_| {
            let res = path.get(step).copied();
            step += 1;
            res
        });

        let collected = acc.collect();
        // Should collect only the straight y-axis coords
        assert_eq!(collected, vec![Coords::new(0, 5), Coords::new(1, 5), Coords::new(2, 5)]);
    }

    #[test]
    fn test_from_peek_fn_break_line() {
        let path = vec![Coords::new(0, 0), Coords::new(0, 1), Coords::new(1, 2), Coords::new(1, 3)];
        let mut step = 0;

        let acc = LineAccumulator::from_peek_fn(|_| {
            let res = path.get(step).copied();
            step += 1;
            res
        });

        let collected = acc.collect();
        // Should stop at first coordinate that breaks the straight line
        assert_eq!(collected, vec![Coords::new(0, 0), Coords::new(0, 1)]);
    }

    #[test]
    fn test_from_peek_fn_empty_path() {
        let path: Vec<Coords> = vec![];
        let mut step = 0;

        let acc = LineAccumulator::from_peek_fn(|_| {
            let res = path.get(step).copied();
            step += 1;
            res
        });

        // Empty path → collected should be empty
        assert!(acc.collect().is_empty());
    }

    #[test]
    fn test_from_peek_fn_single_coords() {
        let path = vec![Coords::new(5, 5)];
        let mut step = 0;

        let acc = LineAccumulator::from_peek_fn(|_| {
            let res = path.get(step).copied();
            step += 1;
            res
        });

        // Only one coordinate → collected should be empty
        assert!(acc.collect().is_empty());
    }
}
