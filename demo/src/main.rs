#![allow(unused)]

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
