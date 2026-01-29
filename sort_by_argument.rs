use cargo_snippet::snippet;

#[snippet("sort_by_argument")]
use std::cmp::Ordering;

#[snippet("sort_by_argument")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginPolicy {
    /// (0,0) を最初に置く（「原点は角度0扱い」みたいにしたい時）
    First,
    /// (0,0) を最後に置く（「とりあえず壊れない」デフォルト向け）
    Last,
    /// (0,0) が来たら panic（原点が出ないはずの問題でバグ検知したい時）
    Forbid,
}

#[snippet("sort_by_argument")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieBreak {
    /// 同偏角（同一直線）なら原点から近い順（norm^2 昇順）
    /// - 全順序（total order）にしやすく、安定で無難
    Norm2Asc,
    /// 同偏角なら (x,y) の辞書順
    /// - “方向”の比較というより「出力の見た目」を揃えたい用途向け
    Lex,
}

#[snippet("sort_by_argument")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgSortOptions {
    pub origin: OriginPolicy,
    pub tie: TieBreak,
}

#[snippet("sort_by_argument")]
impl Default for ArgSortOptions {
    fn default() -> Self {
        // - 多くの問題は (0,0) を含まないが、ライブラリ化すると「混ざる」ケースが起こりがち。
        //   ここで Forbid をデフォルトにすると、汎用利用で突然 panic し得る。
        //   なので「壊れない」Last をデフォルトにしておく。
        //
        // - tie-break を入れないと「同偏角」の点の比較が未定義になり、sort が全順序を要求する場面で不安定になり得る。
        //   norm^2 昇順は直感的で、典型的に困らない。
        Self {
            origin: OriginPolicy::Last,
            tie: TieBreak::Norm2Asc,
        }
    }
}

#[snippet("sort_by_argument")]
#[inline]
fn upper_half(x: i64, y: i64) -> bool {
    // “偏角ソート”はまず「円を2つに割って」から外積で並べるのが定番。
    //
    // 何を上半分にするか（境界の扱い）で結果が変わるので、ここで基準を固定する。
    // - y > 0 を上半平面
    // - y == 0 なら x >= 0（+x軸上）は「上側」に含める
    //
    // これにより、+x軸が最初（角度0）になり、反時計回りに [0, 2π) の順になる。
    y > 0 || (y == 0 && x >= 0)
}

#[snippet("sort_by_argument")]
#[inline]
fn cross(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    // i64 同士の外積は簡単にオーバーフローするので i128 で計算する。
    (ax as i128) * (by as i128) - (ay as i128) * (bx as i128)
}

#[snippet("sort_by_argument")]
#[inline]
fn norm2(x: i64, y: i64) -> i128 {
    // tie-break 用の二乗距離（平方根不要）
    (x as i128) * (x as i128) + (y as i128) * (y as i128)
}

#[snippet("sort_by_argument")]
/// 偏角ソート（argument sort）のための比較関数（i64座標）
///
/// - 基準: +x軸を 0、反時計回りに増加する順（[0, 2π)）
/// - 実装: `atan2` を使わず、半平面判定 + 外積（cross）だけで比較（誤差なし・高速）
/// - 同偏角は tie-break で全順序にする（`sort_by` で安全に使える）
///
/// 典型解説（AtCoder ABC442 Editorial）:
/// - https://atcoder.jp/contests/abc442/editorial/15136
///
/// 注意:
/// - (0,0) は偏角が定義しづらいので `OriginPolicy` で扱いを決める
pub fn cmp_by_argument_i64(a: (i64, i64), b: (i64, i64), opt: ArgSortOptions) -> Ordering {
    let (ax, ay) = a;
    let (bx, by) = b;

    // 原点 (0,0) は「角度」が決まらない（全ての方向でもある）ので、
    // 何も考えずに cross/upper に入れると比較の公理が壊れ得る。
    // → 先にポリシーで処理してしまう。
    let a0 = ax == 0 && ay == 0;
    let b0 = bx == 0 && by == 0;
    if a0 || b0 {
        match opt.origin {
            OriginPolicy::First => return b0.cmp(&a0),
            OriginPolicy::Last => return a0.cmp(&b0),
            OriginPolicy::Forbid => panic!("(0,0) is not allowed in argument sort"),
        }
    }

    // 外積だけで比較すると、角度が 2π で循環するため「全順序」にならない。
    // そこでまず半平面（upper_half）で円を2分割し、循環を断ち切る。
    let ha = upper_half(ax, ay);
    let hb = upper_half(bx, by);
    if ha != hb {
        // ha=true（上半平面側）を先に
        return hb.cmp(&ha);
    }

    // 同じ半平面内では外積で偏角の大小が決まる
    let cr = cross(ax, ay, bx, by);
    if cr != 0 {
        // cr > 0 なら a → b が反時計回り（a の方が角度が小さい）なので Ordering::Less
        return if cr > 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }

    // cross==0 は同一直線（同偏角 or 反対方向）だが、
    // ここまでで半平面は揃っているので「同偏角」と見てよい。
    // このとき tie-break がないと Ordering::Equal が大量に出て不安定になり得る。
    match opt.tie {
        TieBreak::Norm2Asc => norm2(ax, ay).cmp(&norm2(bx, by)),
        TieBreak::Lex => (ax, ay).cmp(&(bx, by)),
    }
}

#[snippet("sort_by_argument")]
/// デフォルト設定（原点は最後、同偏角は距離昇順）での比較
pub fn cmp_by_argument_default_i64(a: (i64, i64), b: (i64, i64)) -> Ordering {
    cmp_by_argument_i64(a, b, ArgSortOptions::default())
}

#[snippet("sort_by_argument")]
/// 任意の点型 T を (i64,i64) に写して偏角ソートする（オプション指定）
pub fn sort_by_argument_i64<T>(
    pts: &mut [T],
    to_xy: impl Fn(&T) -> (i64, i64),
    opt: ArgSortOptions,
) {
    // sort_by は比較が「全順序」(Ordering の公理を満たす) であることを期待する。
    // 上の cmp は half-plane + cross + tie-break で全順序化しているので安全。
    pts.sort_by(|p, q| cmp_by_argument_i64(to_xy(p), to_xy(q), opt));
}

#[snippet("sort_by_argument")]
/// デフォルト設定で偏角ソートする
pub fn sort_by_argument_default_i64<T>(pts: &mut [T], to_xy: impl Fn(&T) -> (i64, i64)) {
    sort_by_argument_i64(pts, to_xy, ArgSortOptions::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_orders_ccw_from_pos_x() {
        // +x(0) -> +y(π/2) -> -x(π) -> -y(3π/2)
        let mut v = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        v.sort_by(|a, b| cmp_by_argument_default_i64(*a, *b));
        assert_eq!(v, vec![(1, 0), (0, 1), (-1, 0), (0, -1)]);
    }

    #[test]
    fn same_direction_tie_break_by_norm2() {
        let mut v = vec![(2, 2), (1, 1), (3, 3)];
        v.sort_by(|a, b| cmp_by_argument_default_i64(*a, *b));
        assert_eq!(v, vec![(1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn default_handles_origin_last() {
        // デフォルトは原点を最後へ
        let mut v = [(0, 0), (1, 0), (0, 1)];
        v.sort_by(|a, b| cmp_by_argument_default_i64(*a, *b));
        assert_eq!(v[v.len() - 1], (0, 0));
    }

    #[test]
    fn custom_options_work() {
        let opt = ArgSortOptions {
            origin: OriginPolicy::First,
            tie: TieBreak::Lex,
        };
        let mut v = [(0, 0), (2, 2), (1, 1)];
        v.sort_by(|a, b| cmp_by_argument_i64(*a, *b, opt));
        assert_eq!(v[0], (0, 0));
    }

    #[test]
    fn matches_reference_sort_using_atan2_for_small_points() {
        // 小さい整数点で atan2 参照ソートと一致することを確認（同偏角は norm2 で tie-break）
        // f64 の比較は厳密性がないので、大きな座標では検証に向かない。
        // ここでは [-3,3] の小さい範囲で「順序の形」が一致するかを見る。
        let mut pts = vec![];
        for x in -3..=3 {
            for y in -3..=3 {
                if x == 0 && y == 0 {
                    continue;
                }
                pts.push((x, y));
            }
        }

        let mut a = pts.clone();
        a.sort_by(|p, q| cmp_by_argument_default_i64(*p, *q));

        // 参照：+x軸基準で [0,2π) に正規化した角 + norm2
        let mut b = pts.clone();
        b.sort_by(|(px, py), (qx, qy)| {
            let ap = (*py as f64).atan2(*px as f64);
            let aq = (*qy as f64).atan2(*qx as f64);
            let ap = if ap < 0.0 {
                ap + std::f64::consts::TAU
            } else {
                ap
            };
            let aq = if aq < 0.0 {
                aq + std::f64::consts::TAU
            } else {
                aq
            };
            ap.partial_cmp(&aq)
                .unwrap()
                .then_with(|| super::norm2(*px, *py).cmp(&super::norm2(*qx, *qy)))
        });

        assert_eq!(a, b);
    }

    #[test]
    fn generic_sort_works_for_custom_type() {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct P {
            x: i64,
            y: i64,
        }

        let mut ps = vec![
            P { x: 0, y: -1 },
            P { x: 1, y: 0 },
            P { x: 0, y: 1 },
            P { x: -1, y: 0 },
        ];

        sort_by_argument_default_i64(&mut ps, |p| (p.x, p.y));

        let got: Vec<(i64, i64)> = ps.iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(got, vec![(1, 0), (0, 1), (-1, 0), (0, -1)]);
    }

    #[test]
    fn origin_forbid_panics() {
        let opt = ArgSortOptions {
            origin: OriginPolicy::Forbid,
            tie: TieBreak::Norm2Asc,
        };
        let res = std::panic::catch_unwind(|| {
            let _ = cmp_by_argument_i64((0, 0), (1, 0), opt);
        });
        assert!(res.is_err());
    }
}
