use crate::factory::config::RootBeanDefinition;

/// 每个 #[Component] 在编译期通过 inventory::submit! 提交一条注册信息。
/// Application::run() 启动时遍历所有条目，自动注册到容器中。
pub struct BeanRegistration {
    /// 返回该 bean 的 BeanDefinition（包含 name、scope、supplier 等）
    pub definition: fn() -> RootBeanDefinition,
}

// 声明全局收集点：inventory 会在链接期将所有 submit! 的条目汇聚到这里
inventory::collect!(BeanRegistration);
