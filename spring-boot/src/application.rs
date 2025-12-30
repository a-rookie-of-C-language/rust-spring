pub struct Application{
    shutdown_hook: SpringApplicationShutdownHook,
    primary_sources: Vec<TypeId>,
    sources: Vec<String>,
    main_application_class:  TypeId,
    resource_loader: ResourceLoader,
    bean_name_generator: BeanNameGenerator,
    envrionment: ConfigurableEnvironment,
    initializers: Vec<ApplicationContextInitializer>,
    listeners: Vec<ApplicationListener>,
    default_properties: HashMap<String,String>,
    bootstrap_registry_initializers: Vec<BootstrapRegistryInitializer>,
    additional_profiles: Vec<String>,
    application_context_factory: ApplicationContextFactory,
    application_startup: ApplicationStartup,
}


impl Application{
    fn new(resource_loader: ResourceLoader,primary_sources: Vec<TypeId>) -> Self{
        self{
            shutdown_hook: SpringApplicationShutdownHook::new(),
            primary_sources,
            sources: Vec::new(),
            main_application_class: TypeId::of::<()>(),
            resource_loader,
            bean_name_generator: DefaultBeanNameGenerator::new(),
            envrionment: StandardEnvironment::new(),
            initializers: Vec::new(),
            listeners: Vec::new(),
            default_properties: HashMap::new(),
            bootstrap_registry_initializers:Vec::new(),
            additional_profiles:Vec::new(),
            application_context_factory: DefaultApplicationContextFactory::new(),
            application_startup: DefaultApplicationStartup::new(),
        }
    }

    pub fn run(&mut self,args: Vec<String>)->ConfigurableApplicationContext{
        let startTime: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let default_bootstrap_context = self.create_bootstrap_context();
    }

    fn create_bootstrap_context(&self)->DefaultBootstrapContext{
        let default = DefaultBootstrapContext::new();
        self.bootstrap_registry_initializers.iter().for_each(|initializer|{
            initializer.initialize(&default);
        });
        default
    }

}