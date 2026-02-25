# rust-spring

> 用 Rust 复刻 Spring Framework 核心——注解驱动的 IoC 容器、依赖注入，以及 Spring Boot 风格的自动配置。不支持 XML，只用注解。

[![CI](https://github.com/arookieofc/rust-spring/actions/workflows/ci.yml/badge.svg)](https://github.com/arookieofc/rust-spring/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg)](https://www.rust-lang.org)

[English](README.md) | 中文

---

## 简介

`rust-spring` 将 Java Spring 生态的核心思想移植到惯用的 Rust 代码中：

| Java Spring | rust-spring |
|---|---|
| `@Component` | `#[Component]` |
| `@Autowired` | `#[autowired]`（字段属性） |
| `@Bean` | `#[Bean]`（标注在函数上） |
| `@Scope("prototype")` | `#[Scope("prototype")]` |
| `@Lazy` | `#[Lazy]` |
| `@Value("${key:default}")` | `#[Value("${key:default}")]` |
| `SpringApplication.run()` | `Application::run()` |
| `application.properties` | `application.properties` |

---

## 项目结构

```
rust-spring/
├── spring-core        # 基础 trait 与工具类
├── spring-beans       # BeanFactory、BeanDefinition、Environment
├── spring-context     # ApplicationContext、bean 生命周期
├── spring-boot        # 应用入口 + 统一 re-export（用户从这里开始）
├── spring-macro       # 过程宏：#[Component]、#[Bean]、#[Value] 等
├── spring-aop         # AOP 模块（开发中）
├── spring-expression  # SpEL 风格表达式引擎（开发中）
├── spring-util        # 通用工具
├── example            # 可运行的完整演示
└── initializer        # CLI 脚手架工具，一键生成新项目
```

用户只需依赖 **`spring-boot`**，其余 crate 全部自动传递引入。

---

## 快速上手

### 添加依赖

```toml
# Cargo.toml
[dependencies]
spring-boot = { git = "https://github.com/a-rookie-of-C-language/rust-spring" }
```

### 编写应用

```rust
use spring_boot::{Application, ApplicationContext, Component, Value};

#[Component]
#[derive(Debug, Default, Clone)]
struct HelloService {
    #[Value("${greeting:Hello, World}")]
    greeting: String,
}

fn main() {
    let context = Application::run();

    if let Some(bean) = context.get_bean("helloService") {
        if let Some(svc) = bean.downcast_ref::<HelloService>() {
            println!("{}", svc.greeting);
        }
    }
}
```

### 在项目根目录添加 `application.properties`

```properties
greeting=Hello from rust-spring!
```

### 运行

```bash
cargo run
```

---

## 脚手架工具

使用 `initializer` CLI 快速生成一个开箱即用的项目：

```bash
# 在 rust-spring 工作区内执行
cargo run -p initializer -- --name my-app --output /tmp
cd /tmp/my-app
cargo run
```

生成的目录结构：

```
my-app/
├── Cargo.toml              # 只有一行 spring-boot git 依赖
├── application.properties  # 示例配置
└── src/
    └── main.rs             # HelloService + AppConfig 演示
```

---

## 注解说明

### `#[Component]`

将结构体标记为受管 bean，rust-spring 在启动时自动注册。

```rust
#[Component]
#[derive(Debug, Default, Clone)]
struct UserService { ... }
```

bean 名称默认为结构体名的 camelCase（`UserService` → `"userService"`）。

---

### `#[autowired]`（字段）

将另一个 bean 注入到字段中，字段类型本身必须也是 `#[Component]`。

```rust
#[Component]
#[derive(Debug, Default, Clone)]
struct OrderService {
    #[autowired]
    user_service: UserService,
}
```

---

### `#[Bean]`

通过工厂函数定义 bean，等价于 Java 的 `@Configuration + @Bean`。

```rust
#[Bean(name = "dataSource")]
fn create_data_source() -> DataSource {
    DataSource { url: "postgres://localhost/mydb".into() }
}
```

---

### `#[Value("${key:default}")]`（字段）

从 `application.properties` 注入配置值，`:` 后面是缺省值。

```rust
#[Component]
#[derive(Debug, Default, Clone)]
struct Config {
    #[Value("${server.port:8080}")]
    port: i32,

    #[Value("${app.name:my-app}")]
    name: String,
}
```

---

### `#[Scope("prototype")]`

每次显式调用 `do_create_bean` 时创建新实例，而不是复用单例缓存。

```rust
#[Component]
#[Scope("prototype")]
#[derive(Debug, Default, Clone)]
struct RequestContext { ... }
```

---

### `#[Lazy]`

延迟初始化：`Application::run()` 时跳过，首次调用 `get_bean` 时才创建。

```rust
#[Component]
#[Lazy]
#[derive(Debug, Default, Clone)]
struct HeavyService { ... }
```

---

## application.properties

将此文件放在二进制文件旁边（`cargo run` 时放在项目根目录）。`Application::run()` 会在装配任何 bean 之前加载它。

```properties
app.name=my-rust-app
server.port=9090
db.url=postgres://localhost/dev
```

---

## 运行示例

```bash
git clone https://github.com/arookieofc/rust-spring.git
cd rust-spring
cargo run -p example
```

预期输出：

```
[Singleton]  person bean: Person { id: 0, name: "" }
[Autowired]  user bean:   User { person: Person { ... }, id: 0, name: "" }
[Prototype]  requestContext: prototype bean (not cached in singleton store)
[Lazy]       heavyService: not yet initialized (lazy=true, needs do_create_bean)
[Lazy]       heavyService initialized: HeavyService { initialized: false }
[Bean]       appConfig: AppConfig { version: "1.0.0", max_connections: 100 }
[Value]      serverConfig: ServerConfig { port: 8080, app_name: "rust-spring", ... }
```

---

## Roadmap

- [x] IoC 容器（`BeanFactory`、`BeanDefinitionRegistry`）
- [x] Singleton 与 Prototype 作用域
- [x] Lazy 懒加载
- [x] `#[autowired]` 字段注入
- [x] `#[Bean]` 工厂函数
- [x] `#[Value]` 从 `application.properties` 注入配置
- [ ] AOP（面向切面编程）
- [ ] SpEL 风格表达式语言
- [ ] 条件 bean（`#[ConditionalOnProperty]`）
- [ ] Spring Data 风格 Repository 抽象
- [ ] HTTP 层（Actix / Axum 集成）

---

## 贡献

请参阅 [CONTRIBUTING.zh.md](CONTRIBUTING.zh.md)。

---

## 许可证

本项目基于 [MIT 协议](LICENSE) 开源。
