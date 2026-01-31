use crate::factory::BeanFactory;

pub trait ConfigurableBeanFactory: BeanFactory {
    fn register_singleton(&mut self, _bean_name: &str, _singleton_object: Box<dyn std::any::Any>) {}
    fn destroy_singleton(&mut self, _bean_name: &str) {}
    fn destroy_singletons(&mut self) {}
}
