/// Global AOP configuration flags.
#[derive(Debug, Clone)]
pub struct AopConfig {
    /// When `true`, the `AopBeanPostProcessor` prints a debug line each time
    /// an advisor is applied to a bean.
    pub debug: bool,
}

impl Default for AopConfig {
    fn default() -> Self {
        AopConfig { debug: false }
    }
}
