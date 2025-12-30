use std::collections::HashMap;
use std::any::Any;
use spring_core::bean::factory::BeanFactory;
use spring_core::registry::BeanDefinitionRegistry;
use spring_beans::{BeanDefinition, BeanScope};
use crate::scanner::component_registry;

/// ApplicationContext trait - 应用上下文接口
pub trait ApplicationContext: BeanFactory + BeanDefinitionRegistry {
    /// 刷新容器
    fn refresh(&mut self);
}

/// 默认的应用上下文实现
pub struct DefaultApplicationContext {
    /// 一级缓存：完全初始化的单例 Bean
    singleton_objects: HashMap<String, Box<dyn Any>>,
    
    /// 二级缓存：早期暴露的 Bean（用于解决循环依赖）
    early_singleton_objects: HashMap<String, Box<dyn Any>>,
    
    /// 三级缓存：Bean 工厂函数
    singleton_factories: HashMap<String, Box<dyn Fn() -> Box<dyn Any>>>,
    
    /// Bean 定义映射
    bean_definition_map: HashMap<String, BeanDefinition>,   
}


impl DefaultApplicationContext {
    pub fn new() -> Self {
        DefaultApplicationContext {
            singleton_objects: HashMap::new(),
            early_singleton_objects: HashMap::new(),
            singleton_factories: HashMap::new(),
            bean_definition_map: HashMap::new(),
        }
    }

    /// 从全局注册表加载所有 BeanDefinition
    pub fn load_bean_definitions(&mut self) {
        for (name, create_definition, create_instance) 
        in component_registry::get_all_components() {
            let bean_definition = create_definition();
            self.bean_definition_map.insert(name.clone(), bean_definition);
            self.singleton_factories.insert(name, create_instance);
        }        
    }

    fn get_or_create_bean(&mut self, name: &str) -> Option<&dyn Any> {
        // 1. 从一级缓存获取
        if self.singleton_objects.contains_key(name) {
            return self.singleton_objects.get(name).map(|b| b.as_ref());
        }
        
        // 2. 从二级缓存获取（处理循环依赖）
        if self.early_singleton_objects.contains_key(name) {
            return self.early_singleton_objects.get(name).map(|b| b.as_ref());
        }
        
        // 3. 获取 BeanDefinition
        let definition = self.bean_definition_map.get(name)?.clone();
        
        // 4. 先创建所有依赖
        for dep_name in &definition.dependencies {
            self.get_or_create_bean(dep_name)?;
        }
        
        // 5. 从工厂创建实例
        let factory = self.singleton_factories.get(name)?;
        let instance = factory();
        
        // 6. 根据作用域处理缓存
        if matches!(definition.scope, BeanScope::Singleton) {
            self.singleton_objects.insert(name.to_string(), instance);
            return self.singleton_objects.get(name).map(|b| b.as_ref());
        }
        
        // Prototype 作用域不缓存（这里需要特殊处理返回值）
        None
    }


}

impl ApplicationContext for DefaultApplicationContext {
    fn refresh(&mut self) {
        println!("🔄 开始刷新容器...");
        
        // 1. 加载所有 BeanDefinition
        self.load_bean_definitions();
        println!("✅ 加载了 {} 个 BeanDefinition", self.bean_definition_map.len());
        
        // 2. 实例化所有非懒加载的单例 Bean
        let bean_names: Vec<String> = self.bean_definition_map.keys().cloned().collect();
        
        for name in bean_names {
            if let Some(definition) = self.bean_definition_map.get(&name) {
                if !definition.is_lazy && matches!(definition.scope, BeanScope::Singleton) {
                    println!("📦 创建 Bean: {}", name);
                    self.get_or_create_bean(&name);
                }
            }
        }
        
        println!("✅ 容器刷新完成！");
    }
}

impl BeanFactory for DefaultApplicationContext {
    fn get_bean(&self, name: &str) -> Option<&dyn Any> {
        self.singleton_objects.get(name)
            .map(|b| b.as_ref())
            .or_else(|| self.early_singleton_objects.get(name).map(|b| b.as_ref()))
    }

    fn is_singleton(&self, name: &str) -> bool {
        self.bean_definition_map
            .get(name)
            .map(|def| matches!(def.scope, BeanScope::Singleton))
            .unwrap_or(false)
    }

    fn contains_bean(&self, name: &str) -> bool {
        self.bean_definition_map.contains_key(name)
    }

    fn do_create_bean(&mut self, name: &str) -> Option<&dyn Any> {
        self.get_or_create_bean(name)
    }
}

impl BeanDefinitionRegistry for DefaultApplicationContext {
    fn register_bean_definition(&mut self, name: &str, bean_definition: BeanDefinition) {
        self.bean_definition_map.insert(name.to_string(), bean_definition);
    }

    fn remove_bean_definition(&mut self, name: &str) {
        self.bean_definition_map.remove(name);
    }

    fn contains_bean_definition(&self, name: &str) -> bool {
        self.bean_definition_map.contains_key(name)
    }

    fn get_bean_definition(&self, name: &str) -> Option<&BeanDefinition> {
        self.bean_definition_map.get(name)
    }

    fn get_bean_definition_names(&self) -> Vec<String> {
        self.bean_definition_map.keys().cloned().collect()
    }
}
