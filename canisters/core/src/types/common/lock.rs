/// 锁定结果
pub enum LockResult<T> {
    /// 锁定成功
    Locked(T),
    /// 异步再试
    Retry(u8),
}
