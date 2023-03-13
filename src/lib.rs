mod arith;
mod impls;

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
    ($p: expr) => {{
        todo!("recursive macro that calls P2::from($p) or P3::from($p) based on the number of elements");
    }};

    ($x: expr, $y: expr) => {
        P2 { x: $x, y: $y }
    };

    ($x: expr, $y: expr, $z: expr) => {
        P3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}
