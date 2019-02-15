#![allow(unused)]

use config_jam::{config, OverrideWith};

mod envy_test;

config! {
/// My Foo config
#[derive(Debug, Default, Clone, OverrideWith, serde::Serialize, serde::Deserialize)]
#[field_derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub config Foo {
    /// A Value for this and that
    /// TODO: Say it's optional. <--> inject default value here!
    pub barval: String = "Foo".into(),
    /// A Test for this and that
    /// TODO:  Say it's optional. <--> inject default value here!
    pub test: u32 = 42,
    /// A Required value for this and that
    /// TODO: Say it is required.    
    pub required: u64
}


// We can add several configs
#[derive(Debug, Default)]
#[field_derive(Debug)]
pub config Bar {
    pub single: &'static str = "Bar",
    // And we can have complex initializers
    pub multiple: Vec<u32> = vec![42, 38]
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

    let bar = Bar::default();
    // Now read-only access with the same structure!
    let bar = bar.get();

    let s = dbg!(bar.single);
    let m = dbg!(bar.multiple);
}
