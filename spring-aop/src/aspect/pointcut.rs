/// Pointcut: resolves `"beanName::methodName"` pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pointcut {
    pub bean_name: String,
    pub method_name: String,
}

impl Pointcut {
    /// Parse a pointcut expression of the form `"beanName::methodName"`.
    ///
    /// # Panics
    /// Panics at runtime if the expression does not contain `::`.
    pub fn parse(expr: &str) -> Self {
        let mut parts = expr.splitn(2, "::");
        let bean_name = parts
            .next()
            .expect("pointcut expression must be 'beanName::methodName'")
            .to_string();
        let method_name = parts
            .next()
            .expect("pointcut expression must be 'beanName::methodName'")
            .to_string();
        Pointcut {
            bean_name,
            method_name,
        }
    }

    /// Returns `true` when this pointcut matches the given bean + method.
    pub fn matches(&self, bean_name: &str, method_name: &str) -> bool {
        self.bean_name == bean_name && self.method_name == method_name
    }
}
