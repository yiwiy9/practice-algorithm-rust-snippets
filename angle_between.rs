use cargo_snippet::snippet;

#[snippet("angle_between")]
#[inline]
fn cross(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (by as i128) - (ay as i128) * (bx as i128)
}

#[snippet("angle_between")]
#[inline]
fn dot(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (bx as i128) + (ay as i128) * (by as i128)
}

#[snippet("angle_between")]
/// 2ベクトル a, b のなす角（0..=PI, ラジアン）を返す。
///
/// - 数値的に安定な atan2(|cross|, dot) を使う（acos より安全）
/// - a or b が (0,0) の場合は角度を 0 とみなす（この問題では通常出ないが安全側）
///
/// # Examples
/// - same direction: 0
/// - right angle: PI/2
/// - opposite direction: PI
pub fn angle_between(a: (i64, i64), b: (i64, i64)) -> f64 {
    let (ax, ay) = a;
    let (bx, by) = b;

    if (ax == 0 && ay == 0) || (bx == 0 && by == 0) {
        return 0.0;
    }

    let cr = cross(ax, ay, bx, by).abs() as f64; // |a×b|
    let dt = dot(ax, ay, bx, by) as f64; // a·b
    cr.atan2(dt) // 0..=PI
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, eps: f64) {
        assert!(
            (a - b).abs() <= eps,
            "not close: a={} b={} diff={}",
            a,
            b,
            (a - b).abs()
        );
    }

    #[test]
    fn angle_between_basic_cases() {
        let eps = 1e-12;
        // 0 deg
        assert_close(angle_between((1, 0), (2, 0)), 0.0, eps);

        // 90 deg
        assert_close(
            angle_between((1, 0), (0, 1)),
            std::f64::consts::FRAC_PI_2,
            eps,
        );

        // 180 deg
        assert_close(angle_between((1, 0), (-1, 0)), std::f64::consts::PI, eps);
    }

    #[test]
    fn angle_between_is_symmetric() {
        let eps = 1e-12;
        let a = (3, 4);
        let b = (-5, 2);
        assert_close(angle_between(a, b), angle_between(b, a), eps);
    }

    #[test]
    fn angle_between_scale_invariant() {
        let eps = 1e-12;
        let a = (2, 1);
        let b = (-1, 3);
        assert_close(angle_between(a, b), angle_between((4, 2), (-2, 6)), eps);
    }

    #[test]
    fn angle_between_handles_zero_vector_as_zero() {
        let eps = 1e-12;
        assert_close(angle_between((0, 0), (1, 0)), 0.0, eps);
        assert_close(angle_between((1, 0), (0, 0)), 0.0, eps);
        assert_close(angle_between((0, 0), (0, 0)), 0.0, eps);
    }
}
