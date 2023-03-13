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
