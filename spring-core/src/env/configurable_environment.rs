pub trait ConfigurableEnvironment {
    fn set_active_profiles(&mut self, profiles: Vec<String>);
    fn add_active_profile(&mut self, profile: String);
    fn get_active_profiles(&self) -> Vec<String>;
}

impl Environment for ConfigurableEnvironment {

}

impl ConfigruablePropertyResolver for ConfigurableEnvironment {

}