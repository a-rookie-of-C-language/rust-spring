use spring_beans::BeanDefinition;

pub trait BeanDefinitionRegistry {
    fn register_bean_definition(&mut self, name: &str, bean_definition: BeanDefinition);
    fn remove_bean_definition(&mut self, name: &str);
    fn contains_bean_definition(&self, name: &str) -> bool;
    fn get_bean_definition(&self, name: &str) -> Option<&BeanDefinition>;
    fn get_bean_definition_names(&self) -> Vec<String>;
}

