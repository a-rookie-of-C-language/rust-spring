pub trait ConfigurableListableBeanFactory {
    fn pre_instantiate_singletons(&mut self);
    fn destroy_singleton(&mut self, name: &str);
    fn destroy_singletons(&mut self);
}


impl ConfigurableBeanFactory for ConfigurableListableBeanFactory {
    
}