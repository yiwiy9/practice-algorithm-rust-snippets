use cargo_snippet::snippet;

#[snippet("vec2_ops")]
#[inline]
fn cross(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (by as i128) - (ay as i128) * (bx as i128)
}

#[snippet("vec2_ops")]
#[inline]
fn dot(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (bx as i128) + (ay as i128) * (by as i128)
}

#[snippet("angle_small")]
#[snippet(include = "vec2_ops")]
/// 2ベクトル a, b のなす角（0..=PI, ラジアン）を返す。
///
/// - 数値的に安定な atan2(|cross|, dot) を使う（acos より安全）
/// - a or b が (0,0) の場合は角度を 0 とみなす（この問題では通常出ないが安全側）
///
/// # Examples
/// - same direction: 0
/// - right angle: PI/2
/// - opposite direction: PI
pub fn angle_small(a: (i64, i64), b: (i64, i64)) -> f64 {
    let (ax, ay) = a;
    let (bx, by) = b;

    if (ax == 0 && ay == 0) || (bx == 0 && by == 0) {
        return 0.0;
    }

    let cr = cross(ax, ay, bx, by).abs() as f64; // |a×b|
    let dt = dot(ax, ay, bx, by) as f64; // a·b
    cr.atan2(dt) // 0..=PI
}

#[snippet("angle_ccw")]
#[snippet(include = "vec2_ops")]
/// ベクトル a から b への「左回り有向角」（-PI..=PI, ラジアン）を返す。
///
/// - 数値的に安定な atan2(cross, dot) を使う（符号付き cross を保持）
/// - cross(a,b) > 0 なら b は a の左側（反時計回り）で、角度は正
/// - cross(a,b) < 0 なら b は a の右側（時計回り）で、角度は負
/// - a or b が (0,0) の場合は角度を 0 とみなす（この問題では通常出ないが安全側）
///
/// # Examples
/// - same direction: 0
/// - left 90 deg: +PI/2
/// - right 90 deg: -PI/2
/// - opposite direction: PI（または -PI）
///
/// # Notes
/// - 0..2PI の「左回り角」が欲しい場合は、戻り値が負なら +TAU して正規化する
pub fn angle_ccw(a: (i64, i64), b: (i64, i64)) -> f64 {
    let (ax, ay) = a;
    let (bx, by) = b;

    if (ax == 0 && ay == 0) || (bx == 0 && by == 0) {
        return 0.0;
    }

    let cr = cross(ax, ay, bx, by) as f64; // signed
    let dt = dot(ax, ay, bx, by) as f64;
    cr.atan2(dt) // -PI..=PI
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
    fn angle_small_basic_cases() {
        let eps = 1e-12;
        // 0 deg
        assert_close(angle_small((1, 0), (2, 0)), 0.0, eps);

        // 90 deg
        assert_close(
            angle_small((1, 0), (0, 1)),
            std::f64::consts::FRAC_PI_2,
            eps,
        );

        // 180 deg
        assert_close(angle_small((1, 0), (-1, 0)), std::f64::consts::PI, eps);
    }

    #[test]
    fn angle_small_is_symmetric() {
        let eps = 1e-12;
        let a = (3, 4);
        let b = (-5, 2);
        assert_close(angle_small(a, b), angle_small(b, a), eps);
    }

    #[test]
    fn angle_small_scale_invariant() {
        let eps = 1e-12;
        let a = (2, 1);
        let b = (-1, 3);
        assert_close(angle_small(a, b), angle_small((4, 2), (-2, 6)), eps);
    }

    #[test]
    fn angle_small_handles_zero_vector_as_zero() {
        let eps = 1e-12;
        assert_close(angle_small((0, 0), (1, 0)), 0.0, eps);
        assert_close(angle_small((1, 0), (0, 0)), 0.0, eps);
        assert_close(angle_small((0, 0), (0, 0)), 0.0, eps);
    }

    #[test]
    fn angle_ccw_basic_cases() {
        let eps = 1e-12;

        // same direction: 0
        assert_close(angle_ccw((1, 0), (2, 0)), 0.0, eps);

        // left 90 deg: +PI/2
        assert_close(angle_ccw((1, 0), (0, 1)), std::f64::consts::FRAC_PI_2, eps);

        // right 90 deg: -PI/2
        assert_close(angle_ccw((0, 1), (1, 0)), -std::f64::consts::FRAC_PI_2, eps);
    }

    #[test]
    fn angle_ccw_opposite_direction_is_pm_pi() {
        let eps = 1e-12;

        // opposite direction: PI (or -PI). Here atan2(0, negative) should give +PI.
        let ang = angle_ccw((1, 0), (-1, 0));
        assert!(
            (ang - std::f64::consts::PI).abs() <= eps || (ang + std::f64::consts::PI).abs() <= eps
        );
    }

    #[test]
    fn angle_ccw_negates_when_swapped() {
        let eps = 1e-12;
        let a = (3, 4);
        let b = (-5, 2);

        let ab = angle_ccw(a, b);
        let ba = angle_ccw(b, a);

        // 通常は ba == -ab。境界の ±PI だけは同符号になり得るので例外扱い。
        if (ab - std::f64::consts::PI).abs() <= eps || (ab + std::f64::consts::PI).abs() <= eps {
            // boundary: allow both to be ±PI
            assert!(
                (ba - std::f64::consts::PI).abs() <= eps
                    || (ba + std::f64::consts::PI).abs() <= eps
            );
        } else {
            assert_close(ba, -ab, eps);
        }
    }

    #[test]
    fn angle_ccw_scale_invariant() {
        let eps = 1e-12;
        let a = (2, 1);
        let b = (-1, 3);
        assert_close(angle_ccw(a, b), angle_ccw((4, 2), (-2, 6)), eps);
    }

    #[test]
    fn angle_ccw_handles_zero_vector_as_zero() {
        let eps = 1e-12;
        assert_close(angle_ccw((0, 0), (1, 0)), 0.0, eps);
        assert_close(angle_ccw((1, 0), (0, 0)), 0.0, eps);
        assert_close(angle_ccw((0, 0), (0, 0)), 0.0, eps);
    }
}
