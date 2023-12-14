use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Index2d {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Grid2d<T> {
    len_x: usize,
    len_y: usize,
    data: Vec<T>,
}

impl<T> Grid2d<T> {
    pub fn new(len_x: usize, len_y: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            len_x,
            len_y,
            data: vec![T::default(); len_x * len_y],
        }
    }

    pub fn is_valid(&self, index: Index2d) -> bool {
        index.x >= 0
            && (index.x as usize) < self.len_x
            && index.y >= 0
            && (index.y as usize) < self.len_y
    }

    fn linearize(&self, index: Index2d) -> usize {
        (index.x as usize) + (index.y as usize) * self.len_x
    }

    pub fn len_x(&self) -> usize {
        self.len_x
    }

    pub fn len_y(&self) -> usize {
        self.len_y
    }
}

impl<T> Index<Index2d> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Index2d) -> &Self::Output {
        let idx = self.linearize(index);
        &self.data[idx]
    }
}

impl<T> IndexMut<Index2d> for Grid2d<T> {
    fn index_mut(&mut self, index: Index2d) -> &mut Self::Output {
        let idx = self.linearize(index);
        &mut self.data[idx]
    }
}
