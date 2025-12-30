pub trait SpringApplicationshutdownHookHandler {
    fn register_shutdown_hook(&mut self);
    fn run_shutdown_hooks(&mut self);
}