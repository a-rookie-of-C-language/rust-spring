pub trait ResourceLoader {
    fn get_resource(&self, location: &str) -> Option<Vec<u8>>;
}