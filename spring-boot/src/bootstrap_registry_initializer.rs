pub trait BootstrapRegistryInitializer {
    fn initialize(&mut self, registry: &mut BootstrapRegister);
}