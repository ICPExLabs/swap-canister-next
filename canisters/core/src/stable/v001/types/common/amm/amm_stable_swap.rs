/// 稳定币做市商（StableSwap AMM）
/// - 核心公式：(x + y)^n = k（n 为调节参数）
/// - 代表项目：Curve、Fei Protocol
/// - 特点
///   - 针对稳定币或同类型资产设计，滑点极低。
///   - 通过「虚拟价格」机制减少套利机会。
///   - Curve 支持 3pool（USDC/USDT/DAI）等复杂池。
