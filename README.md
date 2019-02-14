# config-jam
WORK-IN-PROGRESS, RELEASE SOON :)
## Typesafe, ergonomic config for Rust

```rust
use config_jam::{config, OverrideWith};

config! {
#[derive(Debug, Default, Clone, OverrideWith, serde::Serialize, serde::Deserialize)]
#[field_derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub config Foo {
    pub barval: String = "Foo".into(),
    pub test: u32 = 42,
    pub required: u64
}
}

fn main() {
    let mut foo_base = Foo::default();

    println!("Foo Base:: {:#?}", foo_base.get());

    let foo_toml = Foo {
        barval: Some("Bar".to_string().into()), // TODO improve this
        test: None,
        required: 9000,
    };

    println!("Foo TOML:: {:#?}", foo_toml.get());

    foo_base.override_with(foo_toml);

    println!("Foo Base overriden by Foo TOML:: {:#?}", foo_base.get());

    let foo_env = Foo {
        barval: Some("Bazinga!".to_string().into()), // TODO improve this
        test: Some(98.into()),
        required: 9999,
    };

    println!("Foo ENV {:#?}", foo_env.get());

    foo_base.override_with(foo_env);

    println!(
        "Foo Base overriden by Foo TOML then Foo ENV:: {:#?}",
        foo_base.get()
    );

    // You can now use any field you like
    let some_val = dbg!(foo_base.get().barval);
    // do_something with some_val!
}

```

This program prints the following output:

```
Foo Base:: FooView {
    barval: "Foo",
    test: 42,
    required: 0
}
Foo TOML:: FooView {
    barval: "Bar",
    test: 42,
    required: 9000
}
Foo Base overriden by Foo TOML:: FooView {
    barval: "Bar",
    test: 42,
    required: 9000
}
Foo ENV FooView {
    barval: "Bazinga!",
    test: 98,
    required: 9999
}
Foo Base overriden by Foo TOML then Foo ENV:: FooView {
    barval: "Bazinga!",
    test: 98,
    required: 9999
}
[src/main.rs:48] foo_base.get().barval = "Bazinga!"
```

## Required Rust version

Config Jam works on `Rust` stable.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.