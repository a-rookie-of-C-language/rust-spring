pub struct DefaultApplicationContextFactory{
    
}

impl ApplicationContextFactory for DefaultApplicationContextFactory{
    fn create_application_context(&self) -> Box<dyn ConfigurableApplicationContext> {
        Box::new(GenericApplicationContext::new())
    }
}