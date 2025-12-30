pub trait PropertyResolver {
    fn contains_property(&self, key: &str) -> bool;
    fn get_property(&self, key: &str) -> Option<String>;
}