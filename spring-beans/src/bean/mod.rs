
pub mod bean_post_processor;
pub mod bean_wrapper;

use std::any::TypeId;

/// Bean 的作用域
#[derive(Debug, Clone, PartialEq)]
pub enum BeanScope {
    /// 单例模式 - 容器中只有一个实例
    Singleton,
    /// 原型模式 - 每次获取都创建新实例
    Prototype,
}

/// Bean 定义 - 描述一个 Bean 的元数据
#[derive(Debug, Clone)]
pub struct BeanDefinition {
    /// Bean 名称
    pub name: String,
    /// 类型标识
    pub type_id: TypeId,
    /// 作用域
    pub scope: BeanScope,
    /// 是否懒加载
    pub is_lazy: bool,
    /// 依赖的其他 Bean 名称
    pub dependencies: Vec<String>,
}

/// Component trait - 由 #[component] 宏自动实现
pub trait Component {
    /// 获取 Bean 名称
    fn __bean_name() -> &'static str;
    
    /// 创建 BeanDefinition
    fn __create_bean_definition() -> BeanDefinition;
    
    /// 创建实例
    fn __new_instance() -> Box<dyn std::any::Any>;
}

