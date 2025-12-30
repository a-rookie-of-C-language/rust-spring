pub trait ConfigruablePropertyResolver {
    fn set_ignore_unresolvable_nested_placeholders(&mut self, ignore: bool);
    fn is_ignore_unresolvable_nested_placeholders(&self) -> bool;
}

impl PropertyResolver for ConfigruablePropertyResolver {

}