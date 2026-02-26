use crate::aspect::advice::Advice;
use crate::aspect::pointcut::Pointcut;

/// An `Advisor` binds a `Pointcut` to an `Advice`.
pub struct Advisor {
    pub pointcut: Pointcut,
    pub advice: Advice,
}

impl Advisor {
    pub fn new(pointcut: Pointcut, advice: Advice) -> Self {
        Advisor { pointcut, advice }
    }
}
