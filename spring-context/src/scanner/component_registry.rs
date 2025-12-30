use std::sync::Mutex;
use spring_beans::BeanDefinition;

type ComponentFactory = Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync>;
type DefinitionFactory = Box<dyn Fn() -> BeanDefinition + Send + Sync>;

struct ComponentEntry {
    name: String,
    create_definition: DefinitionFactory,
    create_instance: ComponentFactory,
}

static COMPONENT_REGISTRY: Mutex<Option<Vec<ComponentEntry>>> = Mutex::new(None);

// 由宏生成的代码调用
pub fn __register_component<F, D>(
    name: &str,
    create_definition: D,
    create_instance: F,
)
where
    F: Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static,
    D: Fn() -> BeanDefinition + Send + Sync + 'static,
{
    let mut registry = COMPONENT_REGISTRY.lock().unwrap();
    
    if registry.is_none() {
        *registry = Some(Vec::new());
    }
    
    if let Some(ref mut entries) = *registry {
        entries.push(ComponentEntry {
            name: name.to_string(),
            create_definition: Box::new(create_definition),
            create_instance: Box::new(create_instance),
        });
    }
}

// 供容器调用
pub fn get_all_components() -> Vec<(String, DefinitionFactory, ComponentFactory)> {
    let mut registry = COMPONENT_REGISTRY.lock().unwrap();
    
    if let Some(entries) = registry.take() {
        entries.into_iter()
            .map(|entry| (entry.name, entry.create_definition, entry.create_instance))
            .collect()
    } else {
        Vec::new()
    }
}