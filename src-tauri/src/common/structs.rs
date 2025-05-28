#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}


impl Coords {
    pub fn new(x: u32, y: u32) -> Self {
        Coords { x, y }
    }

    pub fn from_pairs_vector(vec: Vec<[u32; 2]>) -> Vec<Self> {
        vec.into_iter()
            .map(|pair| Coords::new(pair[0], pair[1]))
            .collect()
    }
    
    pub fn to_pair(self) -> [u32; 2] {
        [self.x, self.y]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PathElement {
    pub index: usize,
    pub coords: Coords,
}

