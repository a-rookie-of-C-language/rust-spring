use std::any::{Any, TypeId};
use spring_macro::{all_args_constructor, data};
use super::bean_definition::{BeanDefinition, BeanScope};

#[data]
#[all_args_constructor]
pub struct RootBeanDefinition {
    name: String,
    type_id: TypeId,
    scope: BeanScope,
    is_lazy: bool,
    dependencies: Vec<String>,
    supplier: Box<dyn Fn() -> Box<dyn Any>>,
}


impl BeanDefinition for RootBeanDefinition {
    fn get_bean_class_name(&self) -> &str {
        &self.name
    }

    fn set_scope(&mut self, scope: BeanScope) {
        self.scope = scope;
    }

    fn get_scope(&self) -> BeanScope {
        self.scope
    }

    fn is_lazy_init(&self) -> bool {
        self.is_lazy
    }

    fn set_lazy_init(&mut self, lazy: bool) {
        self.is_lazy = lazy;
    }

    fn get_type_id(&self) -> TypeId {
        self.type_id
    }

    fn has_annotation(&self, annotation: &str) -> bool {
        annotation == "RootBeanDefinition"
    }

    fn create_instance(&self) -> Box<dyn Any> {
        (self.supplier)()
    }

    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
}
