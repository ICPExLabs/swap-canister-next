/// require
#[cfg(feature = "cdk")]
pub fn require<T: AsRef<str>>(condition: bool, message: T) {
    if !condition {
        ic_cdk::trap(message);
    }
}
