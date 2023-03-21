use crate::*;
use num_traits::{cast, NumCast};
use std::{
    any::type_name,
    fmt::Debug,
    ops::{Add, Index, Mul, Neg, Sub},
};

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

    pub fn map<U>(&self, f: fn(&T) -> U) -> P2<U> {
        P2 {
            x: f(&self.x),
            y: f(&self.y),
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
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

    pub fn map<U>(&self, f: fn(&T) -> U) -> P3<U> {
        P3 {
            x: f(&self.x),
            y: f(&self.y),
            z: f(&self.z),
        }
    }
}

impl<T: Copy + Neg<Output = T> + PartialOrd> P2<T> {
    pub fn abs(&self) -> Self {
        let (nx, ny) = (-self.x, -self.y);
        P2 {
            x: if self.x < nx { nx } else { self.x },
            y: if self.y < ny { ny } else { self.y },
        }
    }
}

impl<T: Copy + Neg<Output = T> + PartialOrd> P3<T> {
    pub fn abs(&self) -> Self {
        let (nx, ny, nz) = (-self.x, -self.y, -self.z);
        P3 {
            x: if self.x < nx { nx } else { self.x },
            y: if self.y < ny { ny } else { self.y },
            z: if self.z < nz { nz } else { self.z },
        }
    }
}

impl<T: Copy + Add<Output = T>> P2<T> {
    pub fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T: Copy + Add<Output = T>> P3<T> {
    pub fn sum(&self) -> T {
        self.x + self.y + self.z
    }
}

impl<T: Copy + Mul<Output = T>> P2<T> {
    pub fn product(&self) -> T {
        self.x * self.y
    }
}

impl<T: Copy + Mul<Output = T>> P3<T> {
    pub fn product(&self) -> T {
        self.x * self.y * self.z
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> P2<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> P3<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Copy + Sub<Output = T> + Mul<Output = T> + Default> P2<T> {
    pub fn cross(&self, other: &Self) -> P3<T> {
        P3 {
            z: self.x * other.y - self.y * other.x,
            ..Default::default()
        }
    }
}

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> P3<T> {
    pub fn cross(&self, other: &Self) -> Self {
        P3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Copy + NumCast + Debug> P2<T> {
    pub fn dist(&self, other: &Self) -> f64 {
        let (p1, p2) = (self.as_type::<f64>(), other.as_type::<f64>());
        (p2 - p1).mag()
    }

    pub fn mag(&self) -> f64 {
        let p: P2<f64> = self.as_type();
        (p * p).sum().sqrt()
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let (p1, p2) = (self.as_type::<f64>(), other.as_type::<f64>());
        let mag = p1.mag() * p2.mag();

        match mag == 0f64 {
            true => f64::NAN,
            false => (p1.dot(&p2) / mag).acos(),
        }
    }

    pub fn phase(&self) -> f64 {
        let p: P2<f64> = self.as_type();

        match p.x == 0f64 {
            true => f64::NAN,
            false => (p.y / p.x).atan(),
        }
    }

    pub fn norm(&self) -> P2<f64> {
        let p: P2<f64> = self.as_type();
        let mag = p.mag();

        match mag == 0f64 {
            true => P2::default(),
            false => p / mag,
        }
    }
}

impl<T: Copy + NumCast + Debug> P3<T> {
    pub fn dist(&self, other: &Self) -> f64 {
        let (p1, p2) = (self.as_type::<f64>(), other.as_type::<f64>());
        (p2 - p1).mag()
    }

    pub fn mag(&self) -> f64 {
        let p: P3<f64> = self.as_type();
        (p * p).sum().sqrt()
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let (p1, p2) = (self.as_type::<f64>(), other.as_type::<f64>());
        let mag = p1.mag() * p2.mag();

        match mag == 0f64 {
            true => f64::NAN,
            false => (p1.dot(&p2) / mag).acos(),
        }
    }

    pub fn norm(&self) -> P3<f64> {
        let p: P3<f64> = self.as_type();
        let mag = p.mag();

        match mag == 0f64 {
            true => P3::default(),
            false => p / mag,
        }
    }
}

impl<T> Index<usize> for P2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index `{}` is out of bounds of P2 object", index),
        }
    }
}

impl<T> Index<usize> for P3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index `{}` is out of bounds of P3 object", index),
        }
    }
}

impl<T: Copy + NumCast + Debug> P2<T> {
    #[rustfmt::skip]
    pub fn as_type<U: NumCast>(&self) -> P2<U> {
        P2 {
            x: cast(self.x).unwrap_or_else(|| panic!("value `{:?}` is out of range for `{}`", self.x, type_name::<U>())),
            y: cast(self.y).unwrap_or_else(|| panic!("value `{:?}` is out of range for `{}`", self.y, type_name::<U>())),
        }
    }
}

impl<T: Copy + NumCast + Debug> P3<T> {
    #[rustfmt::skip]
    pub fn as_type<U: NumCast>(&self) -> P3<U> {
        P3 {
            x: cast(self.x).unwrap_or_else(|| panic!("value `{:?}` is out of range for `{}`", self.x, type_name::<U>())),
            y: cast(self.y).unwrap_or_else(|| panic!("value `{:?}` is out of range for `{}`", self.y, type_name::<U>())),
            z: cast(self.z).unwrap_or_else(|| panic!("value `{:?}` is out of range for `{}`", self.z, type_name::<U>())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::{
        consts::{FRAC_PI_2, FRAC_PI_4},
        EPSILON,
    };

    #[test]
    fn indexing() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: 1, y: 2, z: 3 };
        assert!(a[0] == 1 && a[1] == 2);
        assert!(b[0] == 1 && b[1] == 2 && b[2] == 3);
    }

    #[test]
    #[should_panic(expected = "index `2` is out of bounds of P2 object")]
    fn bad_indexing() {
        let a = P2 { x: 1, y: 2 };
        let _ = a[2];
    }

    #[test]
    #[rustfmt::skip]
    fn casting() {
        let a = P2 { x: 1, y: 2 };
        let b = P3 { x: 1, y: 2, z: 3 };
        assert_eq!(a.as_type::<u8>(),  P2 { x: 1,  y: 2 });
        assert_eq!(b.as_type::<f32>(), P3 { x: 1., y: 2., z: 3. });
    }

    #[test]
    #[rustfmt::skip]
    #[should_panic(expected = "value `256` is out of range for `u8`")]
    fn bad_casting() {
        let a = P2 { x: 256, y: 0 };
        let _: P2<u8> = a.as_type();
    }

    #[test]
    #[rustfmt::skip]
    fn equations_2d() {
        let a = P2 { x: 3, y: 4 };
        let b = P2 { x: 6, y: 8 };
        assert!((a.mag()    - 5.).abs() < EPSILON);
        assert!((a.dist(&b) - 5.).abs() < EPSILON);

        let a = P2 { x: 1, y: 1 };
        let b = P2 { x: 1, y: 0 };
        assert!((a.phase()   - FRAC_PI_4).abs() < EPSILON);
        assert!((a.angle(&b) - FRAC_PI_4).abs() < EPSILON);

        let a = P2 { x: 3.,    y: -4.    };
        let b = P2 { x: 3./5., y: -4./5. };
        assert!((a.norm() - b).abs() < P2 { x: EPSILON, y: EPSILON });

        let a = P2 { x: 2, y: 1 };
        let b = P2 { x: 3, y: 7 };
        assert_eq!(a.sum(),     3);
        assert_eq!(b.product(), 21);

        let a = P2 { x: 0, y: 1 };
        let b = P2 { x: 1, y: 9 };
        assert_eq!(a.dot(&b),   9);
        assert_eq!(a.cross(&b), P3 { x: 0, y: 0, z: -1 });
    }

    #[test]
    #[rustfmt::skip]
    fn equations_3d() {
        let a = P3 { x: 2, y: 2, z: 1 };
        let b = P3 { x: 4, y: 4, z: 2 };
        assert!((a.mag()    - 3.).abs() < EPSILON);
        assert!((a.dist(&b) - 3.).abs() < EPSILON);

        let a = P3 { x: 1, y: 1, z:  1 };
        let b = P3 { x: 1, y: 0, z: -1 };
        assert!((a.angle(&b) - FRAC_PI_2).abs() < EPSILON);

        let a = P3 { x: 4.,    y: -4.,    z: 2.    };
        let b = P3 { x: 2./3., y: -2./3., z: 1./3. };
        assert!((a.norm() - b).abs() < P3 { x: EPSILON, y: EPSILON, z: EPSILON});

        let a = P3 { x: 2, y: 1, z: 3 };
        let b = P3 { x: 7, y: 4, z: 9 };
        assert_eq!(a.sum(),     6);
        assert_eq!(b.product(), 252);

        let a = P3 { x: 0, y: 1, z: -1 };
        let b = P3 { x: 1, y: 9, z:  0 };
        assert_eq!(a.dot(&b),   9);
        assert_eq!(a.cross(&b), P3 { x: 9, y: -1, z: -1 });
    }

    #[test]
    fn zero_len_vectors() {
        let a = P2 { x: 0, y: 0 };
        let b = P2 { x: 0, y: 0 };
        assert!(a.phase().is_nan());
        assert!(a.angle(&b).is_nan());
        assert_eq!(a.norm(), P2::default());

        let a = P3 { x: 0, y: 0, z: 0 };
        let b = P3 { x: 0, y: 0, z: 0 };
        assert!(a.angle(&b).is_nan());
        assert_eq!(a.norm(), P3::default());
    }
}
