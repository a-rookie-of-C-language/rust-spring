pub struct Assert;

impl Assert {
    pub fn is_true(condition: bool, message: &str) {
        if !condition {
            panic!("{}", message);
        }
    }

    pub fn not_null<T>(value: Option<T>, message: &str) {
        if value.is_none() {
            panic!("{}", message);
        }
    }

    pub fn has_text(value: &str, message: &str) {
        if value.trim().is_empty() {
            panic!("{}", message);
        }
    }
}