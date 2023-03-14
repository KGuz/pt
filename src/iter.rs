use crate::*;
use std::array;

impl<T: Default> FromIterator<T> for P2<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter().take(2);
        P2 {
            x: iter.next().unwrap_or_default(),
            y: iter.next().unwrap_or_default(),
        }
    }
}

impl<T: Default> FromIterator<T> for P3<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter().take(3);
        P3 {
            x: iter.next().unwrap_or_default(),
            y: iter.next().unwrap_or_default(),
            z: iter.next().unwrap_or_default(),
        }
    }
}

impl<T> IntoIterator for P2<T> {
    type Item = T;
    type IntoIter = array::IntoIter<T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a P2<T> {
    type Item = &'a T;
    type IntoIter = array::IntoIter<&'a T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.x, &self.y].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut P2<T> {
    type Item = &'a mut T;
    type IntoIter = array::IntoIter<&'a mut T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [&mut self.x, &mut self.y].into_iter()
    }
}

impl<T> IntoIterator for P3<T> {
    type Item = T;
    type IntoIter = array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a P3<T> {
    type Item = &'a T;
    type IntoIter = array::IntoIter<&'a T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.x, &self.y, &self.z].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut P3<T> {
    type Item = &'a mut T;
    type IntoIter = array::IntoIter<&'a mut T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [&mut self.x, &mut self.y, &mut self.z].into_iter()
    }
}

// TODO: add unit tests
