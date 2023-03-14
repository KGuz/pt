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

impl<T: Default> From<Vec<T>> for P2<T> {
    fn from(vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), 2, "Invalid length of vector");
        P2::from_iter(vec.into_iter())
    }
}

impl<T: Default + Copy> From<&Vec<T>> for P2<T> {
    fn from(vec: &Vec<T>) -> Self {
        assert_eq!(vec.len(), 2, "Invalid length of vector");
        P2::from_iter(vec.iter().cloned())
    }
}

impl<T: Default + Copy> From<&[T]> for P2<T> {
    fn from(slice: &[T]) -> Self {
        assert_eq!(slice.len(), 2, "Invalid length of slice");
        P2::from_iter(slice.iter().cloned())
    }
}

impl<T: Default> From<Vec<T>> for P3<T> {
    fn from(vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), 3, "Invalid length of vector");
        P3::from_iter(vec.into_iter())
    }
}

impl<T: Default + Copy> From<&Vec<T>> for P3<T> {
    fn from(vec: &Vec<T>) -> Self {
        assert_eq!(vec.len(), 3, "Invalid length of vector");
        P3::from_iter(vec.iter().cloned())
    }
}

impl<T: Default + Copy> From<&[T]> for P3<T> {
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

// TODO: add unit tests
