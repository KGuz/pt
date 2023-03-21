mod arith;
mod conv;
mod impls;
mod iter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct P2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct P3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[macro_export]
macro_rules! pt {
    ($p: expr) => {
        $p.into()
    };

    ($x: expr, $y: expr) => {
        P2::new($x, $y)
    };

    ($x: expr, $y: expr, $z: expr) => {
        P3::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn pt_macro() {
        let a: P2<i32> = pt!(1, 2);
        let b: P3<i32> = pt!(1, 2, 3);

        let _: P2<i32> = pt!(a);
        let _: P3<i32> = pt!(b);

        let _: P2<i32> = pt!((1, 2));
        let _: P3<i32> = pt!((1, 2, 3));

        let _: P2<i32> = pt!([1, 2]);
        let _: P3<i32> = pt!([1, 2, 3]);
    }

    #[test]
    fn readme() {
        let a = pt![1., 1.];
        let b = pt![1., 0.];

        // Basic arithmetic
        let equation = ((1. + b) * 1. - a * a) / 1.;
        assert_eq!(equation, pt![1., 0.]);

        // Linear algebra
        let equation = (a.cross(&b).mag() / a.dot(&b)).atan();
        assert_relative_eq!(equation, a.angle(&b));
    }
}
