// Spring Core 模块 - 核心功能和接口
pub mod bean;
pub mod convert;
pub mod error;
pub mod registry;
pub mod util;

// 重新导出核心类型
pub use registry::{BeanDefinitionRegistry, SingletonBeanRegistry};
