# AutoIt Obfuscator — Rust Web API SDK

Rust client for the [AutoIt Obfuscator](https://www.pelock.com/products/autoit-obfuscator) Web API.

```toml
autoit-obfuscator = "1.5.0"
```

```rust
use autoit_obfuscator::{AutoItObfuscator, AutoItObfuscatorResponse};

#[tokio::main]
async fn main() {
    let client = AutoItObfuscator::new(Some("YOUR-WEB-API-KEY".into()));
    let result = client
        .obfuscate_script_source(r#"ConsoleWrite("Hello World")"#, true)
        .await;
}
```

See `examples/`. Apache-2.0. Copyright Bartosz Wójcik / PELock.
