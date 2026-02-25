use std::collections::HashMap;

/// Abstraction for a named property source (e.g., a `.properties` file, env vars).
pub trait PropertySource {
    fn get_name(&self) -> &str;
    /// Iterate all key-value pairs in this source.
    fn get_properties(&self) -> Vec<(&str, &str)>;
    fn get_property(&self, key: &str) -> Option<&str>;
}

/// Simple in-memory `PropertySource` backed by a `HashMap`.
pub struct MapPropertySource {
    name: String,
    properties: HashMap<String, String>,
}

impl MapPropertySource {
    pub fn new(name: impl Into<String>, properties: HashMap<String, String>) -> Self {
        Self {
            name: name.into(),
            properties,
        }
    }
}

impl PropertySource for MapPropertySource {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_properties(&self) -> Vec<(&str, &str)> {
        self.properties
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}
