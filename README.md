# Representable Macro

Implementation of [representable_interface](https://github.com/scframework/representable_interface)

#### Usage

```rust
#[derive(Representable, Debug)]
struct Os {
    name: String,
    age: u32,
}

fn main() {
    let user = Os {
        name: "Linux".to_string(),
        age: 33,
    };
    println!("{}", user.represent());
}
// Os { name: "Linux", age: 33 }
```

```
[dependencies]
representable_interface = { git = "https://github.com/scframework/representable_interface", version = "0.1.0" }
displayable_macro = { git = "https://github.com/scframework/representable_macro", version = "0.1.0" }
```
