use crate::{P2, P3};

impl<T> P2<T> {
    pub fn new(x: T, y: T) -> Self {
        P2 { x, y }
    }
}

impl<T> P3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        P3 { x, y, z }
    }
}
