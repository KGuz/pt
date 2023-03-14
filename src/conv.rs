use crate::*;

impl<T> From<(T, T)> for P2<T> {
    fn from((x, y): (T, T)) -> Self {
        P2 { x, y }
    }
}

impl<T: Copy> From<&(T, T)> for P2<T> {
    fn from(&(x, y): &(T, T)) -> Self {
        P2 { x, y }
    }
}

impl<T: Copy> From<&mut (T, T)> for P2<T> {
    fn from(&mut (x, y): &mut (T, T)) -> Self {
        P2 { x, y }
    }
}

impl<T> From<(T, T, T)> for P3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        P3 { x, y, z }
    }
}

impl<T: Copy> From<&(T, T, T)> for P3<T> {
    fn from(&(x, y, z): &(T, T, T)) -> Self {
        P3 { x, y, z }
    }
}

impl<T: Copy> From<&mut (T, T, T)> for P3<T> {
    fn from(&mut (x, y, z): &mut (T, T, T)) -> Self {
        P3 { x, y, z }
    }
}

impl<T> From<[T; 2]> for P2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        P2 { x, y }
    }
}

impl<T: Copy> From<&[T; 2]> for P2<T> {
    fn from(&[x, y]: &[T; 2]) -> Self {
        P2 { x, y }
    }
}

impl<T: Copy> From<&mut [T; 2]> for P2<T> {
    fn from(&mut [x, y]: &mut [T; 2]) -> Self {
        P2 { x, y }
    }
}

impl<T> From<[T; 3]> for P3<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        P3 { x, y, z }
    }
}

impl<T: Copy> From<&[T; 3]> for P3<T> {
    fn from(&[x, y, z]: &[T; 3]) -> Self {
        P3 { x, y, z }
    }
}

impl<T: Copy> From<&mut [T; 3]> for P3<T> {
    fn from(&mut [x, y, z]: &mut [T; 3]) -> Self {
        P3 { x, y, z }
    }
}

// TODO: change From to TryFrom for following implementations

impl<T> From<Vec<T>> for P2<T> {
    fn from(vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), 2, "Invalid length of vector");
        P2::from_iter(vec.into_iter())
    }
}

impl<T: Copy> From<&Vec<T>> for P2<T> {
    fn from(vec: &Vec<T>) -> Self {
        assert_eq!(vec.len(), 2, "Invalid length of vector");
        P2::from_iter(vec.iter().cloned())
    }
}

impl<T: Copy> From<&[T]> for P2<T> {
    fn from(slice: &[T]) -> Self {
        assert_eq!(slice.len(), 2, "Invalid length of slice");
        P2::from_iter(slice.iter().cloned())
    }
}

impl<T> From<Vec<T>> for P3<T> {
    fn from(vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), 3, "Invalid length of vector");
        P3::from_iter(vec.into_iter())
    }
}

impl<T: Copy> From<&Vec<T>> for P3<T> {
    fn from(vec: &Vec<T>) -> Self {
        assert_eq!(vec.len(), 3, "Invalid length of vector");
        P3::from_iter(vec.iter().cloned())
    }
}

impl<T: Copy> From<&[T]> for P3<T> {
    fn from(slice: &[T]) -> Self {
        assert_eq!(slice.len(), 3, "Invalid length of slice");
        P3::from_iter(slice.iter().cloned())
    }
}

impl<T> From<P2<T>> for (T, T) {
    fn from(P2 { x, y }: P2<T>) -> Self {
        (x, y)
    }
}

impl<T: Copy> From<&P2<T>> for (T, T) {
    fn from(&P2 { x, y }: &P2<T>) -> Self {
        (x, y)
    }
}

impl<T: Copy> From<&mut P2<T>> for (T, T) {
    fn from(&mut P2 { x, y }: &mut P2<T>) -> Self {
        (x, y)
    }
}

impl<T> From<P3<T>> for (T, T, T) {
    fn from(P3 { x, y, z }: P3<T>) -> Self {
        (x, y, z)
    }
}

impl<T: Copy> From<&P3<T>> for (T, T, T) {
    fn from(&P3 { x, y, z }: &P3<T>) -> Self {
        (x, y, z)
    }
}

impl<T: Copy> From<&mut P3<T>> for (T, T, T) {
    fn from(&mut P3 { x, y, z }: &mut P3<T>) -> Self {
        (x, y, z)
    }
}

impl<T> From<P2<T>> for [T; 2] {
    fn from(P2 { x, y }: P2<T>) -> Self {
        [x, y]
    }
}

impl<T: Copy> From<&P2<T>> for [T; 2] {
    fn from(&P2 { x, y }: &P2<T>) -> Self {
        [x, y]
    }
}

impl<T: Copy> From<&mut P2<T>> for [T; 2] {
    fn from(&mut P2 { x, y }: &mut P2<T>) -> Self {
        [x, y]
    }
}

impl<T> From<P3<T>> for [T; 3] {
    fn from(P3 { x, y, z }: P3<T>) -> Self {
        [x, y, z]
    }
}

impl<T: Copy> From<&P3<T>> for [T; 3] {
    fn from(&P3 { x, y, z }: &P3<T>) -> Self {
        [x, y, z]
    }
}

impl<T: Copy> From<&mut P3<T>> for [T; 3] {
    fn from(&mut P3 { x, y, z }: &mut P3<T>) -> Self {
        [x, y, z]
    }
}

// TODO: implement TryFrom for ndarray::Array1, imageproc::Point behind feature flag

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn from_tuple() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(a, P2::from((1, 2)));
        assert_eq!(b, P3::from((&1, &2, &3)));

        assert_eq!(a, P2::from(&(1, 2)));
        assert_eq!(b, P3::from(&(&1, &2, &3)));

        assert_eq!(a, P2::from(&mut (1, 2)));
        assert_eq!(b, P3::from(&mut (&1, &2, &3)));
    }

    #[test]
    #[rustfmt::skip]
    fn from_array() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(a, P2::from([1, 2]));
        assert_eq!(b, P3::from([&1, &2, &3]));
        assert_eq!(a, P2::from(&[1, 2]));
        assert_eq!(b, P3::from(&[&1, &2, &3]));
        assert_eq!(a, P2::from(&mut [1, 2]));
        assert_eq!(b, P3::from(&mut [&1, &2, &3]));
    }

    #[test]
    #[rustfmt::skip]
    fn into_tuple() {
        let mut a = P2 { x: 1, y: 2 };
        let mut b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(<P2<_> as Into<(_, _)>>::into(a), (1, 2));
        assert_eq!(<P3<&_> as Into<(_, _, _)>>::into(b), (&1, &2, &3));

        assert_eq!(<&P2<_> as Into<(_, _)>>::into(&a), (1, 2));
        assert_eq!(<&P3<&_> as Into<(_, _, _)>>::into(&b), (&1, &2, &3));

        assert_eq!(<&mut P2<_> as Into<(_, _)>>::into(&mut a), (1, 2));
        assert_eq!(<&mut P3<&_> as Into<(_, _, _)>>::into(&mut b), (&1, &2, &3));
    }

    #[test]
    #[rustfmt::skip]
    fn into_array() {
        let mut a = P2 { x: 1, y: 2 };
        let mut b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(<P2<_> as Into<[_; 2]>>::into(a), [1, 2]);
        assert_eq!(<P3<&_> as Into<[_; 3]>>::into(b), [&1, &2, &3]);

        assert_eq!(<&P2<_> as Into<[_; 2]>>::into(&a), [1, 2]);
        assert_eq!(<&P3<&_> as Into<[_; 3]>>::into(&b), [&1, &2, &3]);

        assert_eq!(<&mut P2<_> as Into<[_; 2]>>::into(&mut a), [1, 2]);
        assert_eq!(<&mut P3<&_> as Into<[_; 3]>>::into(&mut b), [&1, &2, &3]);
    }

    #[test]
    #[rustfmt::skip]
    fn from_vec() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(a, P2::from(vec![1, 2]));
        assert_eq!(b, P3::from(vec![&1, &2, &3]));

        assert_eq!(a, P2::from(&vec![1, 2]));
        assert_eq!(b, P3::from(&vec![&1, &2, &3]));
    }

    #[test]
    #[rustfmt::skip]
    fn from_slice() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(a, P2::from([1, 2].as_slice()));
        assert_eq!(b, P3::from([&1, &2, &3].as_slice()));
    }

    #[test]
    #[should_panic(expected = "Invalid length of vector")]
    fn from_bad_vec() {
        let _ = P2::from(vec![1]);
    }

    #[test]
    #[should_panic(expected = "Invalid length of slice")]
    fn from_bad_slice() {
        let _ = P3::from([1].as_slice());
    }
}
