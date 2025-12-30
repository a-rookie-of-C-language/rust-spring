pub trait AutowireCapableBeanFactory {
    fn autowire_bean(&mut self, bean: &mut dyn std::any::Any);
    fn initialize_bean(&mut self, bean_name: &str, bean: &mut dyn std::any::Any) -> ();
}

impl BeanFactory for AutowireCapableBeanFactory {
    
}