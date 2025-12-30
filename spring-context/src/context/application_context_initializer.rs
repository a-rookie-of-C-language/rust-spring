pub trait ApplicationContextInitializer {
    fn initialize<T :ConfigurableApplicationContext>(T : &mut T);
}