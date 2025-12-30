pub struct SimpleBeanDefinitionRegistry {
    bean_definitions: Sync<HashMap<String, BeanDefinition>>,
}

impl BeanDefinitionRegistry for SimpleBeanDefinitionRegistry {
    fn register_bean_definition(&mut self, name: &str, bean_definition: BeanDefinition) {
        let mut definitions = self.bean_definitions.lock().unwrap();
        definitions.insert(name.to_string(), bean_definition);
    }

    fn remove_bean_definition(&mut self, name: &str) {
        let mut definitions = self.bean_definitions.lock().unwrap();
        definitions.remove(name);
    }

    fn contains_bean_definition(&self, name: &str) -> bool {
        let definitions = self.bean_definitions.lock().unwrap();
        definitions.contains_key(name)
    }

    fn get_bean_definition(&self, name: &str) -> Option<&BeanDefinition> {
        let definitions = self.bean_definitions.lock().unwrap();
        definitions.get(name)
    }

    fn get_bean_definition_names(&self) -> Vec<String> {
        let definitions = self.bean_definitions.lock().unwrap();
        definitions.keys().cloned().collect()
    }
}