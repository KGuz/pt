use crate::{P2, P3};

impl<T> P2<T> {
    pub fn new(x: T, y: T) -> Self {
        P2 { x, y }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.x, &self.y].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.x, &mut self.y].into_iter()
    }
}

impl<T> P3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        P3 { x, y, z }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.x, &self.y, &self.z].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.x, &mut self.y, &mut self.z].into_iter()
    }
}
