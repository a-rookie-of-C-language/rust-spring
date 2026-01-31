# rust-spring
## 核心
该项目致力于复刻Java框架spring,包括spring framework的核心功能,如IoC容器,DI依赖注入,AOP面向切面编程等.也进行spring boot的支持.并且进行了一些简化,不再支持繁琐的xml配置,只保留spring boot的注解配置.支持spring boot的自动配置,以及一些常用的组件,如数据库访问,缓存等.

## 为什么写这个项目
因为我想学习一下Rust,并且想复刻一下Java框架spring的核心功能.然后我就想自己实现一个简单的IoC容器,来学习一下Rust的一些特性.以及学习spring框架的优秀思想,也加强我的Rust编程能力,也为了学习spring底层.

## 宏配置
该项目使用了一些自定义的宏,来简化代码的编写.如`#[data]`宏实现了自动生成get,set方法.`#[all_args_constructor]`宏,来自动实现所有参数的构造函数(new()).,`#[no_args_constructor]`宏,来自动实现无参数的构造函数(new_no_args()).等

## 其他
该项目还在不断完善中,欢迎大家参与贡献.
当前已经完成了最小IoC容器的实现,包括BeanFactory,BeanDefinition,BeanDefinitionRegistry等.

## License
该项目基于MIT协议开源,欢迎大家参与贡献.