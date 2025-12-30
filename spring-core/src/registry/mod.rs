// Registry 模块
pub mod bean_definition_registry;
pub mod singleton_bean_registry;

// 重新导出
pub use bean_definition_registry::BeanDefinitionRegistry;
pub use singleton_bean_registry::SingletonBeanRegistry;
