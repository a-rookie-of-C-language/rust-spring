#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeanScope {
    Singleton,
    Prototype,
}

pub trait BeanDefinition {
    fn get_bean_class_name(&self) -> &str;
    fn set_scope(&mut self, scope: BeanScope);
    fn get_scope(&self) -> BeanScope;
    fn is_lazy_init(&self) -> bool;
    fn set_lazy_init(&mut self, lazy: bool);
    fn get_type_id(&self) -> std::any::TypeId;
    fn has_annotation(&self, annotation: &str) -> bool;
    fn create_instance(&self, resolved_deps: &std::collections::HashMap<String, Box<dyn std::any::Any>>, env: &std::collections::HashMap<String, String>) -> Box<dyn std::any::Any>;
    fn get_dependencies(&self) -> Vec<String>;
}
