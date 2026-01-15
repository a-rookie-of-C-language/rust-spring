pub trait ListableBeanFactory: BeanFactory {
    fn contains_bean_definition(&self, name: &str) -> bool;
    fn get_bean_definition_count(&self) -> usize;
    fn get_bean_definition_names(&self) -> Vec<String>;
    fn get_bean_names_for_type<T>(&self,type_id: TypeId) -> Vec<String>;
    fn get_beans_of_type<T: 'static>(&self) -> Vec<Box<T>>;
    fn get_bean_definition_names_for_annotation(&self, annotation: &str) -> Vec<String>;
}
