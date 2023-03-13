use crate::{P2, P3};
use std::ops::*;

macro_rules! impl_op_for {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        // P2
        impl $($path)::+<$ty> for P2<$ty> {
            type Output = P2<$ty>;
            fn $fn(self, other: $ty) -> Self::Output {
                P2 {
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                }
            }
        }
        impl $($path)::+<P2<$ty>> for $ty {
            type Output = P2<$ty>;
            fn $fn(self, other: P2<$ty>) -> Self::Output {
                P2 {
                    x: self.$fn(other.x),
                    y: self.$fn(other.y),
                }
            }
        }
        impl $($path)::+<P2<$ty>> for P2<$ty> {
            type Output = P2<$ty>;
            fn $fn(self, other: P2<$ty>) -> Self::Output {
                P2 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                }
            }
        }
        // P3
        impl $($path)::+<$ty> for P3<$ty> {
            type Output = P3<$ty>;
            fn $fn(self, other: $ty) -> Self::Output {
                P3 {
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                    z: self.z.$fn(other),
                }
            }
        }
        impl $($path)::+<P3<$ty>> for $ty {
            type Output = P3<$ty>;
            fn $fn(self, other: P3<$ty>) -> Self::Output {
                P3 {
                    x: self.$fn(other.x),
                    y: self.$fn(other.y),
                    z: self.$fn(other.z),
                }
            }
        }
        impl $($path)::+<P3<$ty>> for P3<$ty> {
            type Output = P3<$ty>;
            fn $fn(self, other: P3<$ty>) -> Self::Output {
                P3 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                    z: self.z.$fn(other.z),
                }
            }
        }
    };
}

macro_rules! impl_op_assign_for {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        // P2
        impl $($path)::+<$ty> for P2<$ty> {
            fn $fn(&mut self, other: $ty) {
                self.x.$fn(other);
                self.y.$fn(other);
            }
        }
        impl $($path)::+<P2<$ty>> for P2<$ty> {
            fn $fn(&mut self, other: P2<$ty>) {
                self.x.$fn(other.x);
                self.y.$fn(other.y);
            }
        }
        // P3
        impl $($path)::+<$ty> for P3<$ty> {
            fn $fn(&mut self, other: $ty) {
                self.x.$fn(other);
                self.y.$fn(other);
                self.z.$fn(other);
            }
        }
        impl $($path)::+<P3<$ty>> for P3<$ty> {
            fn $fn(&mut self, other: P3<$ty>) {
                self.x.$fn(other.x);
                self.y.$fn(other.y);
                self.z.$fn(other.z);
            }
        }
    };
}

macro_rules! impl_ops {
    ($($ty:ty),*) => {
        $(
            impl_op_for!(Add, add, $ty);
            impl_op_for!(Sub, sub, $ty);
            impl_op_for!(Mul, mul, $ty);
            impl_op_for!(Div, div, $ty);
            impl_op_assign_for!(AddAssign, add_assign, $ty);
            impl_op_assign_for!(SubAssign, sub_assign, $ty);
            impl_op_assign_for!(MulAssign, mul_assign, $ty);
            impl_op_assign_for!(DivAssign, div_assign, $ty);
        )*
    };
}

impl_ops!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn addition() {
        let a: P2<f32> = P2 { x: 10., y: 20. };
        let b: P3<u64> = P3 { x: 10,  y: 20, z: 30 };

        assert_eq!(10. + a + 1. + a, P2 { x: 31., y: 51. });
        assert_eq!(10  + b + 1  + b, P3 { x: 31,  y: 51, z: 71 });

        let mut c: P2<i8>  = P2 { x: 10, y: 20 };
        let mut d: P3<u16> = P3 { x: 10,  y: 20, z: 30 };

        c += 10 + c + 1;
        d += 10 + d + 1;
        
        assert_eq!(c, P2 { x: 31, y: 51 });
        assert_eq!(d, P3 { x: 31, y: 51, z: 71 });
    }

    #[test]
    #[rustfmt::skip]
    fn subtraction() {
        let a: P2<i16> = P2 { x: 10,  y: 20 };
        let b: P3<f64> = P3 { x: 10., y: 20., z: 30. };

        assert_eq!(10  - a - 1  - a, P2 { x: -11,  y: -31 });
        assert_eq!(10. - b - 1. - b, P3 { x: -11., y: -31., z: -51. });

        let mut c: P2<f32> = P2 { x: 10., y: 20. };
        let mut d: P3<i64> = P3 { x: 10,  y: 20, z: 30 };

        c -= 10. - c - 1.;
        d -= 10  - d - 1;
        
        assert_eq!(c, P2 { x: 11., y: 31. });
        assert_eq!(d, P3 { x: 11,  y: 31, z: 51 });
    }

    #[test]
    #[rustfmt::skip]
    fn multiplication() {
        let a: P2<u8>  = P2 { x: 1, y: 2 };
        let b: P3<i32> = P3 { x: 1, y: 2, z: 3 };

        assert_eq!(10 * a * 1 * a, P2 { x: 10, y: 40 });
        assert_eq!(10 * b * 1 * b, P3 { x: 10, y: 40, z: 90 });
        
        let mut c: P2<u128> = P2 { x: 1, y: 2 };
        let mut d: P3<u16>  = P3 { x: 1, y: 2, z: 3 };

        c *= 10 * c * 1;
        d *= 10 * d * 1;

        assert_eq!(c, P2 { x: 10, y: 40 });
        assert_eq!(d, P3 { x: 10, y: 40, z: 90 });
    }

    #[test]
    #[rustfmt::skip]
    fn division() {
        let a: P2<i128> = P2 { x: 1,  y: 2 };
        let b: P3<f32>  = P3 { x: 1., y: 2., z: 5. };

        assert_eq!(10  / a / 1  / a, P2 { x: 10,  y: 2 });
        assert_eq!(10. / b / 1. / b, P3 { x: 10., y: 2.5, z: 0.4 });

        let mut c: P2<f32> = P2 { x: 1., y: 2. };
        let mut d: P3<f64> = P3 { x: 1., y: 2., z: 5. };

        c /= 10. / c / 1.;
        d /= 10. / d / 1.;

        assert_eq!(c, P2 { x: 0.1, y: 0.4 });
        assert_eq!(d, P3 { x: 0.1, y: 0.4, z: 2.5 });
    }
}
