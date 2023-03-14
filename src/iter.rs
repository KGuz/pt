use crate::*;
use std::array;

impl<T> FromIterator<T> for P2<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter().take(2);
        P2 {
            x: iter.next().expect("Invalid length of iter"),
            y: iter.next().expect("Invalid length of iter"),
        }
    }
}

impl<T> FromIterator<T> for P3<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter().take(3);
        P3 {
            x: iter.next().expect("Invalid length of iter"),
            y: iter.next().expect("Invalid length of iter"),
            z: iter.next().expect("Invalid length of iter"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn from_iter() {
        let a = P2 { x: &1, y: &2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(a, P2::from_iter([1, 2].iter()));
        assert_eq!(b, P3::from_iter([1, 2, 3].iter()));

        let c = P2 { x: 1, y: 2 };
        let d = P3 { x: 1, y: 2, z: 3 };

        assert_eq!(c, P2::from_iter([1, 2].into_iter()));
        assert_eq!(d, P3::from_iter([1, 2, 3].into_iter()));
    }

    #[test]
    #[should_panic(expected = "Invalid length of iter")]
    fn from_bad_iter() {
        let _ = P2::from_iter([1].into_iter());
    }

    #[test]
    #[rustfmt::skip]
    fn into_iter() {
        let a = P2 { x: &1, y: &2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert!(a.into_iter().eq([1, 2].iter()));
        assert!(b.into_iter().eq([1, 2, 3].iter()));

        let c = P2 { x: 1, y: 2 };
        let d = P3 { x: 1, y: 2, z: 3 };

        assert!(c.into_iter().eq([1, 2].into_iter()));
        assert!(d.into_iter().eq([1, 2, 3].into_iter()));
    }
}
