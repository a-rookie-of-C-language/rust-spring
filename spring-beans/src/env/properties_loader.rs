use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Loads a Java-style `.properties` file into a `HashMap<String, String>`.
///
/// Format rules:
/// - `key=value` (whitespace around `=` is trimmed)
/// - Lines starting with `#` or `!` are comments and are ignored
/// - Blank lines are ignored
/// - Values may contain `=` characters — only the first `=` is treated as separator
pub struct PropertiesLoader;

impl PropertiesLoader {
    /// Load a `.properties` file from `path`.
    /// Returns `Ok(HashMap)` on success, or an `io::Error` if the file cannot be read.
    pub fn load(path: impl AsRef<Path>) -> std::io::Result<HashMap<String, String>> {
        let content = fs::read_to_string(path)?;
        Ok(Self::parse(&content))
    }

    /// Parse a properties string directly (useful for testing).
    pub fn parse(content: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            // Skip comments and blank lines
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }
            // Split on first '='
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                if !key.is_empty() {
                    map.insert(key, value);
                }
            }
        }
        map
    }
}
