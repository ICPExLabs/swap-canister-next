use std::ops::Range;

/// 比较范围是否涵盖
pub fn is_sub_range(l: &Range<u64>, r: &Range<u64>) -> bool {
    r.start <= l.start && l.end <= r.end
}
