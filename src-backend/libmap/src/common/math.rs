use std::f64::consts::PI;

#[inline]
pub fn cross_prod(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    return x0 * y1 - x1 * y0;
}

#[inline]
pub fn inner_prod(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    return x0 * x1 + y0 * y1;
}

pub const K_MATH_EPSILON: f32 = 1e-5;

#[allow(dead_code)]
pub fn is_within(val: f64, bound1: f64, bound2: f64) -> bool {
    let (mut min, mut max) = (bound1, bound2);
    if bound1 > bound2 {
        (min, max) = (bound2, bound1);
    }
    return val >= min - K_MATH_EPSILON as f64 && val <= max + K_MATH_EPSILON as f64;
}

#[allow(dead_code)]
pub fn is_straight_line(x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    if (x0 == 0.0 && y0 == 0.0) || (x1 == 0.0 && y1 == 0.0) {
        return true;
    }
    let ret = (x0 * x1 + y0 * y1)
        / ((x0 * x0 + y0 * y0).sqrt() * (x1 * x1 + y1 * y1).sqrt());
    // tracing::info!("{}", ret);
    return ret > 0.9999;
}

#[allow(dead_code)]
pub fn normalize_angle(angle: f64) -> f64 {
    let mut a = (angle + PI) % (2.0 * PI);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    return a - PI;
}

pub fn binary_search(host: &Vec<f64>, target: f64) -> usize {
    let mut size = host.len();
    let mut left = 0;
    let mut right = size - 1;
    while left < right {
        let mid = left + size / 2;
        let t = unsafe { host.get_unchecked(mid) };
        if *t < target {
            left = mid + 1;
        } else if *t > target {
            right = mid;
        } else {
            return mid;
        }

        size = right - left;
    }

    if left == host.len() {
        left -= 1;
    }
    return left;
}

#[inline]
pub fn to_2pi(mut a: f32) -> f32 {
    while a > 2.0 * std::f32::consts::PI {
        a = a - 2.0 * std::f32::consts::PI;
    }
    while a < 0.0 {
        a = a + 2.0 * std::f32::consts::PI;
    }
    return a;
}

/// 返回两个角度 中间值
#[allow(dead_code)]
#[inline]
pub fn middle_of_two_radian(a: f64, b: f64) -> f64 {
    let max = a.max(b);
    let min = a.min(b);
    if max - min < PI {
        return min + (max - min) / 2.0;
    } else {
        let t = (min + max) / 2.0;
        return if t > PI {
            (min + max) / 2.0 - PI
        } else {
            (min + max) / 2.0 + PI
        };
    }
}

/// 返回两个角度 夹角
#[inline]
pub fn radian_of_two_radian(a: f64, b: f64) -> f64 {
    let delta = (a - b).abs();
    return delta.min(2.0 * PI - delta);
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::common::math::{middle_of_two_radian, radian_of_two_radian};

    #[test]
    fn it_works() {
        println!(
            "{}, {} ",
            middle_of_two_radian(PI / 10.0, PI / 3.0),
            (PI / 10.0 + PI / 3.0) / 2.0
        );
        println!(
            "{}, {} ",
            middle_of_two_radian(PI / 10.0, PI + PI / 10.0),
            PI + PI / 2.0 + PI / 10.0
        );

        println!("{}, {}", radian_of_two_radian(PI / 2.0, PI / 4.0), PI / 4.0);
        println!(
            "{}, {}",
            radian_of_two_radian(PI / 10.0, PI + PI / 4.0),
            0.75 * PI + PI / 10.0
        );
    }
}
