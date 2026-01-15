// Spring Context 模块 - 应用上下文和容器实现
pub mod context;
pub mod scanner;

// 重新导出核心类型
pub use scanner::component_registry;
