pub struct DefaultListableBeanFactory {
    bean_definition_map: HashMap<String, Box<dyn BeanDefinition>>,
    bean_definition_names: Vec<String>,
    singleton_objects: HashMap<String, Box<dyn Any>>,
    early_singleton_objects: HashMap<String, Box<dyn Any>>,
    singleton_factories: HashMap<String, Box<dyn Fn() -> Box<dyn Any>>>,
    currently_in_creation: HashSet<String>,
    
}


impl ConfigurableBeanFactory for DefaultListableBeanFactory {
    // methods go here
    

}


impl ListableBeanFactory for DefaultListableBeanFactory {
    // methods go here
}