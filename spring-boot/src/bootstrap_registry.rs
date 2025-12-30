pub trait BootstrapRegister{

}

pub trait InstanceSupplier<T> {
    fn get(&self) -> T;
}
