use std::ops::{Add, Mul, Sub};

/// Clamp the given value *val* to lie between the values *low* and *high*.
pub fn clamp_t<T>(val: T, low: T, high: T) -> T
where
    T: PartialOrd,
{
    let r: T;
    if val < low {
        r = low;
    } else if val > high {
        r = high;
    } else {
        r = val;
    }
    r
}

/// Helper function which emulates the behavior of std::upper_bound().
pub fn find_interval<P>(size: i32, pred: P) -> i32
where
    P: Fn(i32) -> bool,
{
    let mut first: i32 = 0;
    let mut len: i32 = size;
    while len > 0 {
        let half = len >> 1;
        let middle = first + half;
        // bisect range based on value of _pred_ at _middle_
        if pred(middle) {
            first = middle + 1;
            len -= half + 1;
        } else {
            len = half;
        }
    }
    clamp_t(first - 1, 0, size - 2)
}

/// Interpolate linearly between two provided values.
pub fn lerp<S, T>(t: S, a: T, b: T) -> T
where
    S: num::One,
    S: Sub<S, Output = S>,
    S: Copy,
    T: Add<T, Output = T>,
    T: Mul<S, Output = T>,
{
    let one: S = num::One::one();
    a * (one - t) + b * t
}
