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
