pub trait ApplicationContextFactory {
    fn create_application_context(&self) -> Box<dyn ConfigurableApplicationContext>;
}