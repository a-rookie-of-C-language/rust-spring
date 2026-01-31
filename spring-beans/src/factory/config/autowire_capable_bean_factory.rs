use crate::factory::BeanFactory;

pub trait AutowireCapableBeanFactory: BeanFactory {
    fn autowire_bean(&mut self, _bean: &mut dyn std::any::Any) {}
    fn initialize_bean(&mut self, _bean_name: &str, _bean: &mut dyn std::any::Any) {}
}
