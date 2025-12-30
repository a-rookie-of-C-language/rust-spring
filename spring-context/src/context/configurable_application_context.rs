pub trait ConfigurableApplicationContext {
    fn refresh(&mut self);
    fn close(&mut self);
    fn is_active(&self) -> bool;
}

impl ApplicationContext for ConfigurableApplicationContext{

}

impl Lifecycle for ConfigurableApplicationContext{

}