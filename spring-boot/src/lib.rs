pub mod application;

pub use application::Application;

// Re-export all proc-macros so users only need `spring-boot` as a dependency.
pub use spring_macro::{Bean, Component, Lazy, Scope, Value, Aspect, Before, After, Around};

// Re-export AOP interceptor so users can call AopProxyRegistry::fire_before / fire_after
pub use spring_aop::{AopProxyRegistry, JoinPoint, AspectRegistration, AdviceKind};

// Re-export the ApplicationContext trait so users can call get_bean / do_create_bean
// without importing spring_context directly.
pub use spring_context::context::application_context::ApplicationContext;
