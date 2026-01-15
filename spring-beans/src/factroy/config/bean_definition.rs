pub trait BeanDefinition {
    fn get_bean_class_name(&self) -> &str;
    fn set_scope(&mut self, scope: &str);
    fn get_scope(&self) -> &str;
    fn is_lazy_init(&self) -> bool;
    fn set_lazy_init(&mut self, lazy: bool);
}