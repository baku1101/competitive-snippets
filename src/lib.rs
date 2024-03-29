pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

pub trait SetMinMaxEq {
    fn setmineq(&mut self, v: Self) -> bool;
    fn setmaxeq(&mut self, v: Self) -> bool;
}
impl<T> SetMinMaxEq for T
where
    T: PartialOrd,
{
    fn setmineq(&mut self, v: T) -> bool {
        *self >= v && {
            *self = v;
            true
        }
    }
    fn setmaxeq(&mut self, v: T) -> bool {
        *self <= v && {
            *self = v;
            true
        }
    }
}

/// 時間計測開始時に一度呼び出し、その後呼ぶと経過時間が秒で返ってくる。
/// 引数にtrueを入れると計測リセット（複数テストケース用）
/// const TL: f64 = 1.95;
/// loop {
///     if get_time(false)/TL >= 1.0 {
///         break;
pub fn get_time(reset_f: bool) -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let sec = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 || reset_f {
            STIME = sec;
        }
        sec - STIME
    }
}
