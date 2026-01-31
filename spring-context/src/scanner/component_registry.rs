use std::sync::{Mutex, OnceLock};

fn registry() -> &'static Mutex<Vec<String>> {
    static REGISTRY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn __register_component<D, F>(name: &str, _definition_supplier: D, _instance_supplier: F)
where
    D: Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static,
    F: Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static,
{
    let mut items = registry().lock().unwrap();
    if !items.contains(&name.to_string()) {
        items.push(name.to_string());
    }
}

pub fn get_all_components() -> Vec<String> {
    registry().lock().unwrap().clone()
}
