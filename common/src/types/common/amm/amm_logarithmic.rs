/// Logarithmic market maker（Logarithmic AMM）
/// - Formula：y = k * ln(x)（x and y are the number of two assets, and k is the constant）
/// - Representative Project：Bancor V3、Balancer（Some pools）
/// - Features
///   - Provides a smoother price curve with slippage below the product of the constant.
///   - Support multi-asset portfolios and adjust liquidity distribution through weights.
///   - Bancor V3 introduces a "elastic supply" mechanism, allowing dynamic additional tokens.
