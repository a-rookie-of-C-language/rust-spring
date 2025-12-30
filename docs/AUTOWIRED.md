# #[autowired] 依赖注入实现说明

## 设计方案

### 问题
Rust 的 proc_macro_attribute 不能直接应用在 struct 字段上，因此 `#[autowired]` 不能像 Java Spring 那样直接标记字段。

### 解决方案
采用**类型推断**方式：`#[component]` 宏自动识别智能指针包装的字段作为依赖。

## 自动依赖识别规则

### 1. 智能指针类型自动识别
任何使用以下智能指针包装的字段都会被自动识别为依赖：
- `Arc<T>`
- `Box<T>`
- `Rc<T>`

```rust
#[component]
struct UserService {
    // ✅ 自动识别为依赖
    user_repo: Arc<UserRepository>,
    
    // ✅ 自动识别为依赖
    cache: Box<Cache>,
    
    // ❌ 不会识别（不是智能指针）
    config: String,
}
```

### 2. 排除基础类型
即使使用智能指针，基础类型不会被识别为 Bean 依赖：
- `String`, `str`
- 数字类型：`i32`, `u64`, `f64` 等
- 标准集合：`Vec`, `HashMap` 等

```rust
#[component]
struct Service {
    // ❌ 不会作为 Bean 依赖
    data: Arc<Vec<String>>,
    
    // ✅ 会作为 Bean 依赖
    repository: Arc<MyRepository>,
}
```

## 使用示例

### 基本用法
```rust
use spring_macro::component;
use std::sync::Arc;

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct Database {
    url: String,
}

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct UserRepository {
    // 自动识别 Database 为依赖
    db: Arc<Database>,
}

#[component(lazy = false, scope = "Singleton")]
#[derive(Default)]
struct UserService {
    // 自动识别 UserRepository 为依赖
    repo: Arc<UserRepository>,
}
```

### 依赖关系
生成的 `BeanDefinition` 会包含依赖信息：
```rust
BeanDefinition {
    name: "UserService",
    dependencies: vec!["UserRepository"],  // 自动提取
    ...
}
```

### 容器自动解析
```rust
fn main() {
    let mut context = DefaultApplicationContext::new();
    context.refresh();  // 自动按依赖顺序初始化
    
    // 获取 Bean（已注入依赖）
    let service = context.get_bean("UserService");
}
```

## 实现细节

### 宏展开示例
```rust
// 源代码
#[component]
struct UserService {
    repo: Arc<UserRepository>,
}

// 展开后
struct UserService {
    repo: Arc<UserRepository>,
}

impl UserService {
    pub fn __create_bean_definition() -> BeanDefinition {
        BeanDefinition {
            name: "UserService".to_string(),
            dependencies: vec!["UserRepository".to_string()],  // 自动提取
            // ...
        }
    }
}

#[ctor::ctor]
fn __register_UserService() {
    component_registry::__register_component(
        "UserService",
        || UserService::__create_bean_definition(),
        || Box::new(UserService::default())
    );
}
```

## 依赖提取逻辑

```rust
fn extract_dependencies(fields: &Fields) -> Vec<String> {
    // 1. 检查字段类型是否为智能指针（Arc, Box, Rc）
    // 2. 提取泛型参数 T
    // 3. 排除基础类型
    // 4. 返回类型名称作为依赖
}
```

## 优势

1. **零注解**：无需手动标记 `#[autowired]`
2. **类型安全**：编译期检查类型
3. **自动推断**：根据类型自动识别依赖
4. **简洁清晰**：代码更简洁

## 限制

1. **必须使用智能指针**：普通引用无法识别
2. **命名匹配**：Bean 名称必须与类型名称一致
3. **需要 Default trait**：当前实现依赖 Default 初始化

## 未来改进

1. ✅ 支持自动依赖识别（已完成）
2. ⏳ 支持构造函数注入
3. ⏳ 支持循环依赖检测
4. ⏳ 支持 Qualifier 限定符
5. ⏳ 支持可选依赖（Option<Arc<T>>）

## 与 Java Spring 对比

| 特性 | Java Spring | Rust Spring |
|------|-------------|-------------|
| 字段注入 | `@Autowired` | 自动识别 `Arc<T>` |
| 构造器注入 | `@Autowired` | 待实现 |
| Setter 注入 | `@Autowired` | 待实现 |
| 可选依赖 | `required=false` | `Option<Arc<T>>` (待实现) |
| 限定符 | `@Qualifier` | 待实现 |
