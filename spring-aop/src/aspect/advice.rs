/// Metadata available inside a `#[Before]` or `#[After]` advice function.
#[derive(Debug, Clone)]
pub struct JoinPoint {
    /// Name of the bean being intercepted (e.g. `"userService"`).
    pub bean_name: String,
    /// Name of the method being intercepted (e.g. `"save"`).
    pub method_name: String,
}

impl JoinPoint {
    pub fn new(bean_name: &str, method_name: &str) -> Self {
        JoinPoint {
            bean_name: bean_name.to_string(),
            method_name: method_name.to_string(),
        }
    }
}

/// The type of advice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceKind {
    Before,
    After,
    Around,
}

/// A single advice: its kind + the function to invoke.
///
/// `Before` / `After` receive a `&JoinPoint`.
/// `Around` advice receives control and must call `proceed` explicitly (see
/// `ProceedingJoinPoint`).  Because Rust closures + `Box<dyn Any>` make a
/// fully-generic proceed tricky without boxing the return value, `Around`
/// advice is modelled as a `fn(&JoinPoint) -> ()` that runs *before* the real
/// method.  An explicit `post_around` hook runs after.  This gives
/// before/after semantics without requiring the advice to call proceed.
pub struct Advice {
    pub kind: AdviceKind,
    pub handler: Box<dyn Fn(&JoinPoint) + Send + Sync>,
}

impl Advice {
    pub fn before(f: impl Fn(&JoinPoint) + Send + Sync + 'static) -> Self {
        Advice {
            kind: AdviceKind::Before,
            handler: Box::new(f),
        }
    }

    pub fn after(f: impl Fn(&JoinPoint) + Send + Sync + 'static) -> Self {
        Advice {
            kind: AdviceKind::After,
            handler: Box::new(f),
        }
    }

    /// Create an `Around` advice.  The handler is called **before** the target
    /// method; a second call (registered separately) happens **after**.
    pub fn around(f: impl Fn(&JoinPoint) + Send + Sync + 'static) -> Self {
        Advice {
            kind: AdviceKind::Around,
            handler: Box::new(f),
        }
    }
}
