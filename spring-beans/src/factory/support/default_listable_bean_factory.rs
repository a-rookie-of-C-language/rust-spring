use std::any::Any;
use std::collections::{HashMap, HashSet};
use spring_macro::data;
use crate::factory::BeanDefinitionRegistry;
use crate::factory::config::{BeanDefinition, ConfigurableBeanFactory};
use crate::factory::BeanFactory;
use crate::factory::listable_bean_factory::ListableBeanFactory;

#[data]
pub struct DefaultListableBeanFactory {
    bean_definition_map: HashMap<String, Box<dyn BeanDefinition>>,
    bean_definition_names: Vec<String>,
    singleton_objects: HashMap<String, Box<dyn Any>>,
    early_singleton_objects: HashMap<String, Box<dyn Any>>,
    singleton_factories: HashMap<String, Box<dyn Fn() -> Box<dyn Any>>>,
    currently_in_creation: HashSet<String>,
}

impl BeanDefinitionRegistry for DefaultListableBeanFactory {
    fn contains_bean_definition(&self, bean_name: &str) -> bool {
        self.bean_definition_map.contains_key(bean_name)
    }

    fn get_bean_definition(&self, bean_name: &str) -> Option<&Box<dyn BeanDefinition>> {
        self.bean_definition_map.get(bean_name)
    }

    fn get_bean_definition_count(&self) -> usize {
        self.bean_definition_map.len()
    }

    fn get_bean_definition_names(&self) -> &Vec<String> {
        &self.bean_definition_names
    }

    fn is_bean_name_in_use(&self, bean_name: &str) -> bool {
        BeanDefinitionRegistry::contains_bean_definition(self, bean_name)
            || self.singleton_objects.contains_key(bean_name)
            || self.early_singleton_objects.contains_key(bean_name)
            || self.singleton_factories.contains_key(bean_name)
            || self.currently_in_creation.contains(bean_name)
    }

    fn register_bean_definition(&mut self, bean_name: &str, bean_definition: Box<dyn BeanDefinition>) {
        self.bean_definition_map.insert(bean_name.to_string(), bean_definition);
        self.bean_definition_names.push(bean_name.to_string());
    }

    fn remove_bean_definition(&mut self, bean_name: &str) {
        self.bean_definition_map.remove(bean_name);
        self.bean_definition_names
            .retain(|n| n != bean_name);
    }
}

impl BeanFactory for DefaultListableBeanFactory {
    fn get_bean(&self, name: &str) -> Option<&dyn Any> {
        self.singleton_objects
            .get(name)
            .or_else(|| self.early_singleton_objects.get(name))
            .map(|boxed| boxed.as_ref())
    }

    fn is_singleton(&self, name: &str) -> bool {
        self.singleton_objects.contains_key(name)
    }

    fn contains_bean(&self, name: &str) -> bool {
        self.bean_definition_map.contains_key(name)
            || self.singleton_objects.contains_key(name)
            || self.early_singleton_objects.contains_key(name)
    }

    fn do_create_bean(&mut self, name: &str) -> Option<&dyn std::any::Any> {
        if self.singleton_objects.contains_key(name) {
            return self.singleton_objects.get(name).map(|boxed| boxed.as_ref());
        }
        if self.early_singleton_objects.contains_key(name) {
            return self.early_singleton_objects.get(name).map(|boxed| boxed.as_ref());
        }
        let definition = self.bean_definition_map.get(name)?;
        let scope = definition.get_scope().to_string();
        let instance = definition.create_instance();
        if scope.eq_ignore_ascii_case("singleton") {
            self.singleton_objects.insert(name.to_string(), instance);
            return self.singleton_objects.get(name).map(|boxed| boxed.as_ref());
        }
        self.early_singleton_objects.insert(name.to_string(), instance);
        self.early_singleton_objects.get(name).map(|boxed| boxed.as_ref())
    }
}


impl ConfigurableBeanFactory for DefaultListableBeanFactory {
    fn register_singleton(&mut self, bean_name: &str, singleton_object: Box<dyn Any>) {
        self.singleton_objects
            .insert(bean_name.to_string(), singleton_object);
    }

    fn destroy_singleton(&mut self, bean_name: &str) {
        self.singleton_objects.remove(bean_name);
    }

    fn destroy_singletons(&mut self) {
        self.singleton_objects.clear();
        self.early_singleton_objects.clear();
        self.singleton_factories.clear();
        self.currently_in_creation.clear();
    }
}


impl ListableBeanFactory for DefaultListableBeanFactory {
    fn contains_bean_definition(&self, name: &str) -> bool {
        self.bean_definition_map.contains_key(name)
    }

    fn get_bean_definition_count(&self) -> usize {
        self.bean_definition_map.len()
    }

    fn get_bean_definition_names(&self) -> Vec<String> {
        self.bean_definition_names.clone()
    }

    fn get_bean_names_for_type<T>(&self, type_id: std::any::TypeId) -> Vec<String> {
      self.bean_definition_map.iter()
            .filter(|(_, bd)| bd.as_ref().get_type_id() == type_id)
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>()
    }

    fn get_beans_of_type<T: 'static>(&self) -> Vec<&T> {
        self.singleton_objects
            .iter()
            .filter_map(|(_, obj)| obj.as_ref().downcast_ref::<T>())
            .collect::<Vec<_>>()
    }

    fn get_bean_definition_names_for_annotation(&self, annotation: &str) -> Vec<String> {
        self.bean_definition_map.iter()
            .filter(|(_, bd)| bd.as_ref().has_annotation(annotation))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>()
    }
}

impl DefaultListableBeanFactory {
    pub fn new() -> Self {
        Self {
            bean_definition_map: HashMap::new(),
            bean_definition_names: Vec::new(),
            singleton_objects: HashMap::new(),
            early_singleton_objects: HashMap::new(),
            singleton_factories: HashMap::new(),
            currently_in_creation: HashSet::new(),
        }
    }
}

impl Default for DefaultListableBeanFactory {
    fn default() -> Self {
        Self::new()
    }
}
