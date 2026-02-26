use crate::aspect::advice::{Advice, AdviceKind, JoinPoint};
use crate::aspect::advisor::Advisor;
use crate::aspect::pointcut::Pointcut;
use std::sync::{Mutex, OnceLock};

/// Global registry of all `Advisor`s collected from `#[Aspect]` classes.
///
/// `spring-macro` submits `AspectRegistration` entries at link time via
/// `inventory`.  `AopProxyRegistry::initialize()` is called once by
/// `Application::run()` to convert those entries into `Advisor`s stored here.
static REGISTRY: OnceLock<Mutex<Vec<Advisor>>> = OnceLock::new();

fn registry() -> &'static Mutex<Vec<Advisor>> {
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

pub struct AopProxyRegistry;

impl AopProxyRegistry {
    /// Register an advisor programmatically (used by `Application::run()` after
    /// converting `AspectRegistration` inventory entries).
    pub fn register(advisor: Advisor) {
        registry().lock().unwrap().push(advisor);
    }

    /// Convenience: register a `Before` advice for `"beanName::methodName"`.
    pub fn register_before(expr: &str, f: impl Fn(&JoinPoint) + Send + Sync + 'static) {
        let pc = Pointcut::parse(expr);
        Self::register(Advisor::new(pc, Advice::before(f)));
    }

    /// Convenience: register an `After` advice for `"beanName::methodName"`.
    pub fn register_after(expr: &str, f: impl Fn(&JoinPoint) + Send + Sync + 'static) {
        let pc = Pointcut::parse(expr);
        Self::register(Advisor::new(pc, Advice::after(f)));
    }

    /// Convenience: register an `Around` advice for `"beanName::methodName"`.
    pub fn register_around(expr: &str, f: impl Fn(&JoinPoint) + Send + Sync + 'static) {
        let pc = Pointcut::parse(expr);
        Self::register(Advisor::new(pc, Advice::around(f)));
    }

    /// Call all `Before` (and `Around` pre-) advices that match
    /// `(bean_name, method_name)`.
    pub fn fire_before(bean_name: &str, method_name: &str) {
        let jp = JoinPoint::new(bean_name, method_name);
        let advisors = registry().lock().unwrap();
        for advisor in advisors.iter() {
            if advisor.pointcut.matches(bean_name, method_name) {
                match advisor.advice.kind {
                    AdviceKind::Before | AdviceKind::Around => {
                        (advisor.advice.handler)(&jp);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Call all `After` (and `Around` post-) advices that match
    /// `(bean_name, method_name)`.
    pub fn fire_after(bean_name: &str, method_name: &str) {
        let jp = JoinPoint::new(bean_name, method_name);
        let advisors = registry().lock().unwrap();
        for advisor in advisors.iter() {
            if advisor.pointcut.matches(bean_name, method_name) {
                match advisor.advice.kind {
                    AdviceKind::After | AdviceKind::Around => {
                        (advisor.advice.handler)(&jp);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Returns `true` if any advisor targets the given bean.
    pub fn has_advisors_for(bean_name: &str) -> bool {
        registry()
            .lock()
            .unwrap()
            .iter()
            .any(|a| a.pointcut.bean_name == bean_name)
    }
}
