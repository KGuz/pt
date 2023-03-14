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

impl<T> TryFrom<Vec<T>> for P2<T> {
    type Error = &'static str;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match vec.len() {
            2 => Ok(P2::from_iter(vec.into_iter())),
            _ => Err("Invalid length of vector"),
        }
    }
}

impl<T: Copy> TryFrom<&Vec<T>> for P2<T> {
    type Error = &'static str;
    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        match vec.len() {
            2 => Ok(P2::from_iter(vec.iter().cloned())),
            _ => Err("Invalid length of vector"),
        }
    }
}

impl<T: Copy> TryFrom<&[T]> for P2<T> {
    type Error = &'static str;
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        match slice.len() {
            2 => Ok(P2::from_iter(slice.iter().cloned())),
            _ => Err("Invalid length of slice"),
        }
    }
}

impl<T> TryFrom<Vec<T>> for P3<T> {
    type Error = &'static str;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match vec.len() {
            3 => Ok(P3::from_iter(vec.into_iter())),
            _ => Err("Invalid length of vector"),
        }
    }
}

impl<T: Copy> TryFrom<&Vec<T>> for P3<T> {
    type Error = &'static str;
    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        match vec.len() {
            3 => Ok(P3::from_iter(vec.iter().cloned())),
            _ => Err("Invalid length of vector"),
        }
    }
}

impl<T: Copy> TryFrom<&[T]> for P3<T> {
    type Error = &'static str;
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        match slice.len() {
            3 => Ok(P3::from_iter(slice.iter().cloned())),
            _ => Err("Invalid length of slice"),
        }
    }
}

// TODO: implement TryFrom for ndarray::Array1, imageproc::Point behind feature flag

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
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<P2<_> as Into<(_, _)>>::into(a), (1, 2));
        assert_eq!(<P3<&_> as Into<(_, _, _)>>::into(b), (&1, &2, &3));
        
        let a = &P2 { x: 1, y: 2 };
        let b = &P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<&P2<_> as Into<(_, _)>>::into(a), (1, 2));
        assert_eq!(<&P3<&_> as Into<(_, _, _)>>::into(b), (&1, &2, &3));

        let a = &mut P2 { x: 1, y: 2 };
        let b = &mut P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<&mut P2<_> as Into<(_, _)>>::into(a), (1, 2));
        assert_eq!(<&mut P3<&_> as Into<(_, _, _)>>::into(b), (&1, &2, &3));
    }

    #[test]
    #[rustfmt::skip]
    fn into_array() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<P2<_> as Into<[_; 2]>>::into(a), [1, 2]);
        assert_eq!(<P3<&_> as Into<[_; 3]>>::into(b), [&1, &2, &3]);

        let a = &P2 { x: 1, y: 2 };
        let b = &P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<&P2<_> as Into<[_; 2]>>::into(a), [1, 2]);
        assert_eq!(<&P3<&_> as Into<[_; 3]>>::into(b), [&1, &2, &3]);

        let a = &mut P2 { x: 1, y: 2 };
        let b = &mut P3 { x: &1, y: &2, z: &3 };
        assert_eq!(<&mut P2<_> as Into<[_; 2]>>::into(a), [1, 2]);
        assert_eq!(<&mut P3<&_> as Into<[_; 3]>>::into(b), [&1, &2, &3]);
    }

    #[test]
    #[rustfmt::skip]
    fn try_from_vec() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(Ok(a), P2::try_from(vec![1, 2]));
        assert_eq!(Ok(b), P3::try_from(vec![&1, &2, &3]));

        assert_eq!(Ok(a), P2::try_from(&vec![1, 2]));
        assert_eq!(Ok(b), P3::try_from(&vec![&1, &2, &3]));

        assert_eq!(Err("Invalid length of vector"), P2::try_from(vec![1]));
        assert_eq!(Err("Invalid length of vector"), P3::try_from(vec![&1, &2]));
    }

    #[test]
    #[rustfmt::skip]
    fn from_slice() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: &1, y: &2, z: &3 };

        assert_eq!(Ok(a), P2::try_from([1, 2].as_slice()));
        assert_eq!(Ok(b), P3::try_from([&1, &2, &3].as_slice()));

        assert_eq!(Err("Invalid length of slice"), P2::try_from([1].as_slice()));
        assert_eq!(Err("Invalid length of slice"), P3::try_from([&1, &2].as_slice()));
    }
}
