// https://stackoverflow.com/a/54035801
pub fn index_math(n: usize, dn: i32) -> Option<usize> {
    if dn.is_negative() {
        if n > 0 {
            // we know dn will never be > 1
            n.checked_sub(dn.wrapping_abs() as u32 as usize)
        } else {
            None
        }
    } else {
        n.checked_add(dn as usize)
    }
}
