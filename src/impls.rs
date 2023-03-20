use crate::*;
use num_traits::{cast, NumCast};
use std::ops::{Add, Index, Mul, Sub};

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

impl<T: Copy + NumCast> P2<T> {
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
        (p.y / p.x).atan()
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

impl<T: Copy + NumCast> P3<T> {
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
            _ => panic!("index '{}' is out of bounds of P2 object", index),
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
            _ => panic!("index '{}' is out of bounds of P3 object", index),
        }
    }
}

impl<T: Copy + NumCast> P2<T> {
    pub fn as_type<U: NumCast>(&self) -> P2<U> {
        P2 {
            x: cast(self.x).unwrap(),
            y: cast(self.y).unwrap(),
        }
    }
}

impl<T: Copy + NumCast> P3<T> {
    pub fn as_type<U: NumCast>(&self) -> P3<U> {
        P3 {
            x: cast(self.x).unwrap(),
            y: cast(self.y).unwrap(),
            z: cast(self.z).unwrap(),
        }
    }
}
