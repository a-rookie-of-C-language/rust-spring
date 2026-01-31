use crate::factory::config::ConfigurableBeanFactory;

pub trait ConfigurableListableBeanFactory: ConfigurableBeanFactory {
    fn pre_instantiate_singletons(&mut self) {}
    fn destroy_singleton(&mut self, _name: &str) {}
    fn destroy_singletons(&mut self) {}
}
