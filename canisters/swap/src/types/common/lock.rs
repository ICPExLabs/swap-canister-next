/// Lock results
pub enum LockResult<T> {
    /// Locked successfully
    Locked(T),
    /// Try again asynchronously
    Retry(u8),
}
